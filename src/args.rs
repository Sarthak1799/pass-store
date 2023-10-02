use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    name = "pass-store",
    author = "Sarthak S",
    version = "0.1.0",
    about = "A CLI based password manager built in rust"
)]
#[command(bin_name = "pass_store")]
pub struct InputArgs {
    #[clap(subcommand)]
    pub input: InputType,
}

#[derive(Debug, Subcommand)]
pub enum InputType {
    /// Initialise pass-store directory
    Init(InitCommand),
    /// Store a new password
    Store(StoreCommands),
    /// Retrives a stored password
    Get(GetCommand),
    /// remove an existing password
    Remove(RemoveCommands),
}

#[derive(Debug, Args)]
pub struct StoreCommands {
    /// path to store password
    pub path: String,
    /// the password to store
    pub password: String,
    #[arg(short = 'u')]
    /// optional field - username
    pub username: Option<String>,
    #[arg(short = 'c')]
    ///  optional field - additional comments to store
    pub comments: Option<String>,
}

#[derive(Debug, Args)]
pub struct RemoveCommands {
    /// path to remove password
    pub path: String,
}

#[derive(Debug, Args)]
pub struct InitCommand {
    /// path to remove password
    pub path: String,
}

#[derive(Debug, Args)]
pub struct GetCommand {
    /// path to retrive password
    pub path: String,
}
