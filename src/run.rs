use clap::{command, Parser, Subcommand};
use serde::Serialize;

use crate::{
    cli::{
        account::{self, AccountCommand, AccountNamespaceResult},
        block::{self, BlockCommand, BlockNamespaceResult},
        transaction::{self, TransactionCommand, TransactionNamespaceResult},
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

#[derive(Debug)]
pub enum CliResult {
    BlockNamespace(BlockNamespaceResult),
    AccountNamespace(AccountNamespaceResult),
    TransactionNamespace(TransactionNamespaceResult),
}

impl Serialize for CliResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            CliResult::BlockNamespace(res) => res.serialize(serializer),
            CliResult::AccountNamespace(res) => res.serialize(serializer),
            CliResult::TransactionNamespace(res) => res.serialize(serializer),
        }
    }
}

pub enum OutputFormat {
    Console,
    Json,
}

fn format_output<T: Serialize>(input: T, format: OutputFormat) -> anyhow::Result<()> {
    match format {
        OutputFormat::Console => println!("{}", serde_json::to_string_pretty(&input)?),
        OutputFormat::Json => todo!(),
    }

    Ok(())
}

pub fn run() -> Result<(), anyhow::Error> {
    let cli = EntryPoint::parse();

    let config_overrides = ConfigOverrides::new(cli.priv_key, cli.rpc_url, cli.config_file);

    let config = get_config(config_overrides)?;

    let execution_context = CommandExecutionContext::new(config)?;

    let res = match cli.command {
        Command::Block(cmd) => block::parse(&execution_context, cmd).map(CliResult::BlockNamespace),
        Command::Account(cmd) => {
            account::parse(&execution_context, cmd).map(CliResult::AccountNamespace)
        }
        Command::Transaction(cmd) => {
            transaction::parse(&execution_context, cmd).map(CliResult::TransactionNamespace)
        }
        Command::Event(_) => todo!(),
        Command::Gas(_) => todo!(),
        Command::Utils(_) => todo!(),
    }?;

    format_output(res, OutputFormat::Console)
}
