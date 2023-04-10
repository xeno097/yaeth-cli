use clap::{command, Parser, Subcommand};

use crate::{
    cli::{
        account::{self, AccountCommand},
        block::{self, BlockCommand},
        transaction::{self, TransactionCommand},
    },
    config::{get_config, ConfigOverrides},
    context::CommandExecutionContext,
};

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
    #[command()]
    Block(BlockCommand),

    /// Execute account related operations
    #[command()]
    Account(AccountCommand),

    /// Execute transaction related operations
    Transaction(TransactionCommand),

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

    let config_overrides = ConfigOverrides::new(cli.priv_key, cli.rpc_url, cli.config_file);

    let config = get_config(config_overrides)?;

    let execution_context = CommandExecutionContext::new(config)?;

    match cli.command {
        Command::Block(cmd) => block::parse(&execution_context, cmd),
        Command::Account(cmd) => account::parse(&execution_context, cmd),
        Command::Transaction(cmd) => transaction::parse(&execution_context, cmd),
        Command::Event(_) => todo!(),
        Command::Gas(_) => todo!(),
        Command::Utils(_) => todo!(),
    }
}
