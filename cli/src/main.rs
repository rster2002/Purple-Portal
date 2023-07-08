use std::env;
use std::path::PathBuf;
use crate::models::input::cli_root::{ClientCommand, RootCommand, ServerCommand};
use clap::Parser;
use client::PurplePortalClient;
use server::PurplePortalServer;
use crate::tokio_fs_adapter::TokioFsAdapter;

mod models;
mod tokio_fs_adapter;

#[tokio::main]
async fn main() {
    let command = RootCommand::parse();

    match command {
        RootCommand::Server(server_command) => {
            match server_command {
                ServerCommand::Start(options) => {
                    let server = PurplePortalServer::new(
                        "a",
                        "b",
                        10,
                        "./".into(),
                    );

                    server.start()
                        .await;
                }
                ServerCommand::Remove(_) => {}
            }
        }
        RootCommand::Client(client_command) => {
            match client_command {
                ClientCommand::Watch(watch_options) => {
                    todo!()
                },
                ClientCommand::Sync(sync_options) => {
                    let vault_path = std::env::current_dir()
                        .expect("Failed to read current path")
                        .join(sync_options.path);

                    let adapter = TokioFsAdapter;

                    let client = PurplePortalClient::init(vault_path, adapter)
                        .await
                        .expect("Failed to start client");

                    let _ = client.run_sync()
                        .await;
                },
            }
        }
    }
}
