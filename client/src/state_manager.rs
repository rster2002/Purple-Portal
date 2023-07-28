use std::collections::HashMap;
use std::ops::Range;
use std::path::{Path, PathBuf};
use diamond_types::list::encoding::EncodeOptions;
use diamond_types::list::{Branch, OpLog};
use diamond_types::{AgentId, LocalVersion};
use similar::{Change, ChangeTag, TextDiff};
use thiserror::Error;
use uuid::{uuid, Uuid};

use crate::recursive_read_dir::RecursiveReadDir;
use crate::models::local_state::LocalOpLog;
use crate::models::ws_messages::{WsClientIncoming, WsClientOutgoing};
use crate::prelude::*;
use crate::PurplePortalClient;
use crate::traits::fs_adapter::FsAdapter;
use crate::traits::ws_client::WsClient;
use crate::utils::diff::Diff;

/// Responsible for updating the local op logs.
pub struct StateManager<'a, T, C>
    where T: FsAdapter,
          C: WsClient<WsClientOutgoing, WsClientIncoming>,
{
    client: &'a PurplePortalClient<T, C>,
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Divergent content detected")]
    DivergentContent,
}

impl<'a, T, C> StateManager<'a, T, C>
    where T: FsAdapter,
          C: WsClient<WsClientOutgoing, WsClientIncoming>,
{
    pub fn new(client: &'a PurplePortalClient<T, C>) -> Self {
        Self {
            client,
        }
    }

    fn diff_root_path(&self) -> PathBuf {
        self.client
            .config_root
            .join("bin")
    }

    pub async fn diff_all(&self) -> Result<Vec<LocalOpLog>> {
        let reader = RecursiveReadDir::new(self.client);
        let paths = reader.walk(&self.client.vault_root)
            .await?
            .into_iter()
            .filter(|p| {
                p.to_str().unwrap().ends_with(".md")
            });

        let mut logs = vec![];

        for path in paths {
            let local_op_log = self.diff_path(&path)
                .await?;

            logs.push(local_op_log);
        }

        Ok(logs)
    }

    pub async fn diff_path(&self, abs_path: &PathBuf) -> Result<LocalOpLog> {
        let relative_path = abs_path.strip_prefix(&self.client.vault_root)
            .unwrap()
            .to_path_buf();

        let mut bin_path = relative_path.clone();
        let _ = bin_path.set_extension("bin");

        let diff_path = self.diff_root_path()
            .join(bin_path);

        if !diff_path.exists() {
            let mut log = OpLog::new();
            let agent_id = Uuid::new_v4().to_string();
            let agent = log.get_or_create_agent_id(&*agent_id);

            let current_content = String::from_utf8(
                self.client.fs_adapter
                    .read_file(abs_path)
                    .await?
            )?;

            if current_content.is_empty() {
                let _ = log.add_insert(agent, 0, &*current_content);
            }

            self.client.fs_adapter
                .create_dir_all(
                    &diff_path
                        .parent()
                        .expect("Diff path should always have a parent")
                        .to_path_buf()
                )
                .await?;

            let local_op_log = LocalOpLog {
                agent_id,
                last_sync: None,
                op_log: log.encode(EncodeOptions::default()),
            };

            let encoded = bincode::serialize(&local_op_log)?;

            self.client.fs_adapter
                .write_file(&diff_path, &encoded)
                .await?;

            return Ok(local_op_log);
        } else {
            let file_content = self.client.fs_adapter
                .read_file(&diff_path)
                .await?;

            let local_log: LocalOpLog = bincode::deserialize(&file_content)?;

            let mut op_log = OpLog::load_from(&local_log.op_log)?;
            let agent = op_log.get_or_create_agent_id(&*local_log.agent_id);

            let current_content = String::from_utf8(
                self.client.fs_adapter
                    .read_file(abs_path)
                    .await?
            )?;

            Self::apply_to_op_log(
                &mut op_log,
                agent,
                current_content,
            )?;

            let new_local = LocalOpLog {
                agent_id: local_log.agent_id,
                last_sync: local_log.last_sync,
                op_log: op_log.encode(EncodeOptions::default()),
            };

            self.client.fs_adapter
                .write_file(&diff_path, &bincode::serialize(&new_local)?)
                .await?;

            return Ok(new_local);
        }
    }

    fn apply_to_op_log(op_log: &mut OpLog, agent: AgentId, content: String) -> Result<()> {
        let mut branch = Branch::new_at_tip(&op_log);
        let branch_content = branch.content().to_string();

        let diff: Vec<Change<&str>> = TextDiff::from_chars(&branch_content, &content)
            .iter_all_changes()
            .collect();

        let delete_items = diff.iter()
            .filter(|x| matches!(x.tag(), ChangeTag::Delete));

        let mut removed_indexes = vec![];
        for item in delete_items {
            let index = item.old_index().unwrap();

            let removed_offset = removed_indexes.iter()
                .filter(|x| &index > x)
                .count();

            let final_index = index - removed_offset;

            let _ = branch.delete_without_content(op_log, agent, final_index..(final_index + 1));
            removed_indexes.push(final_index);
        }

        let insert_items = diff.iter()
            .filter(|x| matches!(x.tag(), ChangeTag::Insert));

        for item in insert_items {
            let index = item.new_index().unwrap();
            let _ = branch.insert(op_log, agent, index, item.value());
        }

        branch.merge(&op_log, op_log.local_version_ref());

        let op_log_content = Branch::new_at_tip(&op_log).content().to_string();

        if op_log_content != content {
            return Err(StateError::DivergentContent.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use diamond_types::list::encoding::EncodeOptions;
    use diamond_types::list::OpLog;
    use crate::state_manager::StateManager;
    use crate::tests::{TestFsAdapter, TestWsClient};

    #[test]
    fn diffs_are_applied_correctly() {
        let cases = [
            ("abcde", "agbdf"),
            ("abc", "defghi"),
            ("a", "bcd"),
            ("abc", "e"),
            ("", "abcdef"),
            ("abc", "abcde"),
            ("ace", "abcde"),
            ("bd", "acde"),
            ("cool right", "crash"),
            ("something cool right?", "something that should not crash"),
        ];

        for case in cases {
            dbg!(&case);

            let mut op_log = OpLog::new();
            let agent = op_log.get_or_create_agent_id("abc");

            if !case.0.is_empty() {
                let _ = op_log.add_insert(agent, 0, case.0);
            }

            let mut op_log = OpLog::load_from(&*op_log.encode(EncodeOptions::default()))
                .unwrap();

            let agent = op_log.get_or_create_agent_id("abc");

            let result = StateManager::<TestFsAdapter, TestWsClient>::apply_to_op_log(&mut op_log, agent, case.1.to_string());
            assert!(result.is_ok());
        }
    }
}
