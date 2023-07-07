use std::path::PathBuf;
use clap::{Subcommand, Args, Parser};

#[derive(Debug, Parser)]
pub enum RootCommand {
    /// Commands related to hosting a server.
    #[clap(subcommand)]
    Server(ServerCommand),

    /// Commands related to interacting with a server.
    #[clap(subcommand)]
    Client(ClientCommand)
}

#[derive(Debug, Subcommand)]
pub enum ServerCommand {
    /// Starts the server and makes everything ready to start accepting connections.
    Start(ServerStartOptions),

    /// Removes all data related to the server installation.
    Remove(ServerRemoveOptions),
}

#[derive(Debug, Args)]
pub struct ServerStartOptions {
    /// The port to start the server on.
    #[arg(short, long)]
    pub port: Option<u16>,
}

#[derive(Debug, Args)]
pub struct ServerRemoveOptions {

}

#[derive(Debug, Subcommand)]
pub enum ClientCommand {
    /// Start watching the target Vault.
    Watch(WatchOptions),

    /// Perform a single sync with the server.
    Sync(WatchOptions),
}

#[derive(Debug, Args)]
pub struct WatchOptions {
    /// The path of the root of the Vault to watch.
    pub path: PathBuf,
}
