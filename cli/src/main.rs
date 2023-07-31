use crate::models::input::cli_root::{ClientCommand, RootCommand, ServerCommand};
use crate::tokio_fs_adapter::TokioFsAdapter;
use crate::tungstenite_ws_client::TungsteniteWsClient;
use clap::Parser;
use client::traits::ws_client::WsClient;
use client::PurplePortalClient;
use server::PurplePortalServer;
use std::env;
use std::path::PathBuf;

mod models;
mod tokio_fs_adapter;
mod tungstenite_ws_client;

#[tokio::main]
async fn main() {
    let command = RootCommand::parse();

    match command {
        RootCommand::Server(server_command) => match server_command {
            ServerCommand::Start(options) => {
                let mut server = PurplePortalServer::new();

                server.start().await;
            }
            ServerCommand::Remove(_) => {}
        },
        RootCommand::Client(client_command) => match client_command {
            ClientCommand::Watch(watch_options) => {
                todo!()
            }
            ClientCommand::Sync(sync_options) => {
                let vault_path = std::env::current_dir()
                    .expect("Failed to read current path")
                    .join(sync_options.path);

                let adapter = TokioFsAdapter;

                dbg!("here");

                let ws_client = TungsteniteWsClient::connect(sync_options.remote_addr)
                    .await
                    .unwrap();

                dbg!("here");

                let client = PurplePortalClient::init(vault_path, adapter, ws_client)
                    .await
                    .expect("Failed to start client");

                let result = client.run_sync().await;

                dbg!(result);
            }
        },
    }
}
