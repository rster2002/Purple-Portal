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
use crate::prelude::*;
use crate::PurplePortalClient;
use crate::traits::fs_adapter::FsAdapter;
use crate::utils::diff::Diff;

pub struct StateManager<'a, T>
    where T: FsAdapter,
{
    client: &'a PurplePortalClient<T>,
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Divergent content detected")]
    DivergentContent,
}

impl<'a, T> StateManager<'a, T>
    where T: FsAdapter
{
    pub fn new(client: &'a PurplePortalClient<T>) -> Self {
        Self {
            client,
        }
    }

    fn diff_root_path(&self) -> PathBuf {
        self.client
            .config_root
            .join("bin")
    }

    pub async fn diff_all(&self) -> Result<()> {
        let reader = RecursiveReadDir::new(self.client);
        let paths = reader.walk(&self.client.vault_root)
            .await?
            .into_iter()
            .filter(|p| {
                p.to_str().unwrap().ends_with(".md")
            });

        for path in paths {
            let i = self.diff_path(&path)
                .await?;
        }

        Ok(())
    }

    pub async fn diff_path(&self, abs_path: &PathBuf) -> Result<()> {
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

            let encoded = bincode::serialize(&LocalOpLog {
                agent_id,
                last_sync: None,
                op_log: log.encode(EncodeOptions::default()),
            })?;

            self.client.fs_adapter
                .write_file(&diff_path, &encoded)
                .await?;
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
        }

        Ok(())
    }

    fn apply_to_op_log(op_log: &mut OpLog, agent: AgentId, content: String) -> Result<()> {
        let mut branch = Branch::new_at_tip(&op_log);
        let branch_content = branch.content().to_string();

        let diff: Vec<Change<&str>> = TextDiff::from_chars(&branch_content, &content)
            .iter_all_changes()
            .collect();

        let mut added_indexes = vec![];
        let mut removed_indexes = vec![];
        for item in diff {
            match item.tag() {
                ChangeTag::Equal => {}
                ChangeTag::Delete => {
                    let index = item.old_index()
                        .expect("Deleted items should always have an old index");

                    let removed_offset = removed_indexes.iter()
                        .filter(|x| &index > x)
                        .count();

                    let added_offset = added_indexes.iter()
                        .filter(|x| &index > x)
                        .count();

                    let final_index = index - removed_offset + added_offset;

                    let _ = branch.delete_without_content(op_log, agent, final_index..(final_index + 1));
                    removed_indexes.push(index);
                }
                ChangeTag::Insert => {
                    let index = item.new_index()
                        .expect("Inserted items should always have a new index");

                    let removed_offset = removed_indexes.iter()
                        .filter(|x| &index > x)
                        .count();

                    let added_offset = added_indexes.iter()
                        .filter(|x| &index > x)
                        .count();

                    let final_index = if added_indexes.len() < removed_indexes.len() {
                        index - removed_offset + added_offset
                    } else {
                        index
                    };

                    let _ = branch.insert(op_log, agent, final_index, item.value());
                    added_indexes.push(index);
                }
            }
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
    use crate::tests::TestFsAdapter;

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

            let result = StateManager::<TestFsAdapter>::apply_to_op_log(&mut op_log, agent, case.1.to_string());
            assert!(result.is_ok());
        }
    }
}
