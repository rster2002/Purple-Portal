use std::env;
use crate::models::input::cli_root::{ClientCommand, RootCommand, ServerCommand};
use clap::Parser;
use client::PurplePortalClient;

mod models;

#[tokio::main]
async fn main() {
    let command = RootCommand::parse();

    match command {
        RootCommand::Server(server_command) => {
            match server_command {
                ServerCommand::Start(options) => {

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


                    let client = PurplePortalClient::init(vault_path)
                        .await
                        .expect("Failed to start client");

                    let _ = client.run_sync()
                        .await;
                },
            }
        }
    }
}
