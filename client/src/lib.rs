#![deny(unused_results)]
#![deny(clippy::unwrap_used)]

use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use serde::Serialize;
use uuid::Uuid;
use crate::models::client_info::ClientInfo;

use crate::models::ws_messages::{WsClientIncoming, WsClientOutgoing};
use crate::prelude::*;
use crate::state_manager::StateManager;
use crate::traits::fs_adapter::FsAdapter;
use crate::traits::ws_client::WsClient;

pub mod error;
pub mod models;
pub(crate) mod prelude;
mod recursive_read_dir;
pub mod state_manager;
pub mod traits;
pub(crate) mod utils;

#[cfg(test)]
mod tests;
mod shared;

pub struct PurplePortalClient<T, C>
where
    T: FsAdapter,
    C: WsClient<WsClientOutgoing, WsClientIncoming>,
{
    pub(crate) vault_root: PathBuf,
    pub(crate) config_root: PathBuf,
    pub(crate) client_info: ClientInfo,
    pub(crate) fs_adapter: T,
    pub(crate) ws_client: C,
}

impl<T, C> PurplePortalClient<T, C>
where
    T: FsAdapter,
    C: WsClient<WsClientOutgoing, WsClientIncoming>,
{
    pub async fn init(vault_root: PathBuf, fs_adapter: T, mut ws_client: C) -> Result<Self> {
        let config_root = vault_root.join(".purple-portal");

        wrap_ws_error!(fs_adapter.create_dir_all(&config_root).await)?;

        let client_info_path = config_root.join("client.bin");
        let client_info: ClientInfo = if fs_adapter.exists(&client_info_path).await? {
            let binary = fs_adapter.read_file(&client_info_path)
                .await?;

            bincode::deserialize(&binary)?
        } else {
            let local_info = ClientInfo {
                id: Uuid::new_v4().to_string(),
            };

            fs_adapter.write_file(&client_info_path, &bincode::serialize(&local_info)?)
                .await?;

            local_info
        };

        wrap_ws_error!(ws_client
            .send(WsClientOutgoing::Authenticate {
                client_info: client_info.clone(),
                password: "abc".to_string(),
            })
            .await)?;

        let received = wrap_ws_error!(ws_client.receive().await)?;

        let WsClientIncoming::AuthenticationSuccess = received else {
            return Err(Error::SocketAuthenticationFailed);
        };

        Ok(Self {
            vault_root,
            config_root,
            client_info,
            fs_adapter,
            ws_client,
        })
    }

    pub fn get_vault_name(&self) -> String {
        self.vault_root
            .file_name()
            .expect("File name should've be checked when creating the client")
            .to_str()
            .expect("Failed to convert to str")
            .to_string()
    }

    pub async fn run_sync(&mut self) -> Result<()> {
        let state_manager = StateManager::new(self);

        let changed_logs = state_manager.diff_all()
            .await?;

        for log in changed_logs {
            wrap_ws_error!(
                self.ws_client.send(WsClientOutgoing::Sync(log))
                    .await
            );
        }

        Ok(())
    }

    pub async fn sync_all(&mut self) -> Result<()> {
        let state_manager = StateManager::new(self);
        let changed_logs = state_manager.get_all_local()
            .await?;

        for log in changed_logs {
            wrap_ws_error!(
                self.ws_client.send(WsClientOutgoing::Sync(log))
                    .await
            )?;
        }

        Ok(())
    }

    pub async fn watch() -> () {
        todo!()
    }
}
