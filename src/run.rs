use clap::{command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct EntryPoint {
    /// Private key to use for signing transactions
    #[arg(short, long)]
    priv_key: Option<String>,

    /// Rpc url to send requests to
    #[arg(short, long)]
    rpc_url: Option<String>,

    /// Optional configuration file
    #[arg(short, long)]
    config_file: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
#[command()]
enum Command {
    /// Execute block related operations
    #[command(subcommand)]
    Block(NoSubCommand),

    /// Execute account related operations
    #[command(subcommand)]
    Account(NoSubCommand),

    /// Execute transaction related operations
    #[command(subcommand)]
    Transaction(NoSubCommand),

    /// Execute event related operations
    #[command(subcommand)]
    Event(NoSubCommand),

    /// Execute gas related operations
    #[command(subcommand)]
    Gas(NoSubCommand),

    /// Collection of utils
    #[command(subcommand)]
    Utils(NoSubCommand),
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum NoSubCommand {}

pub fn run() -> Result<(), anyhow::Error> {
    let cli = EntryPoint::parse();

    match cli.command {
        Command::Block(_) => todo!(),
        Command::Account(_) => todo!(),
        Command::Transaction(_) => todo!(),
        Command::Event(_) => todo!(),
        Command::Gas(_) => todo!(),
        Command::Utils(_) => todo!(),
    }
}