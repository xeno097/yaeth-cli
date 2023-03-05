use clap::{command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct EntryPoint {
    /// Name of the person to greet
    #[command(subcommand)]
    command: Command,
}

/// Doc comment
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
