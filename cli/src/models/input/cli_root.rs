use clap::{Subcommand, Args};

#[derive(Debug, Subcommand)]
pub enum RootCommand {
    /// Commands related to hosting a server.
    #[clap(subcommand)]
    Server(ServerCommand),

    /// Commands related to interacting with a server.
    #[clap(subcommand)]
    Client(ClientOptions)
}

#[derive(Debug, Subcommand)]
pub enum ServerCommand {
    /// Starts the server and makes everything ready to start accepting connections.
    #[clap(subcommand)]
    Start(ServerStartOptions),

    /// Removes all data related to the server installation.
    #[clap(subcommand)]
    Remove(ServerRemoveOptions),
}

#[derive(Debug, Args)]
pub struct ServerStartOptions {
    /// The port to start the server on.
    #[arg(short, long)]
    port: u16,
}

#[derive(Debug)]
pub struct ServerRemoveOptions {

}

#[derive(Debug, Subcommand)]
pub struct ClientOptions {

}
