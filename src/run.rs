use std::fs::File;

use clap::{builder::PossibleValue, command, Parser, Subcommand, ValueEnum};
use serde::Serialize;

use crate::{
    cli::{
        account::{self, AccountCommand, AccountNamespaceResult},
        block::{self, BlockCommand, BlockNamespaceResult},
        gas::{self, GasCommand, GasNamespaceResult},
        transaction::{self, TransactionCommand, TransactionNamespaceResult},
    },
    config::{get_config, ConfigOverrides},
    context::CommandExecutionContext,
};

#[derive(Parser, Debug)]
#[command(
    author = "xeno097",
    about = "An ether-rs wrapper to query the ethereum blockchain from a terminal",
    display_name = "yaeth",
    disable_help_subcommand = true,
    version
)]
struct EntryPoint {
    /// Private key to use for signing transactions
    #[arg(short, long)]
    priv_key: Option<String>,

    /// Rpc url to send requests to
    #[arg(short, long)]
    rpc_url: Option<String>,

    /// Output format for the cli result
    #[arg(short, long, default_value = "console")]
    out: OutputFormat,

    /// Optional name for the output file
    #[arg(short, long, default_value = "out")]
    file: String,

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
    Gas(GasCommand),

    /// Collection of utils
    #[command(subcommand)]
    Utils(NoSubCommand),
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum NoSubCommand {}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CliResult {
    BlockNamespace(BlockNamespaceResult),
    AccountNamespace(AccountNamespaceResult),
    TransactionNamespace(TransactionNamespaceResult),
    GasNamespace(GasNamespaceResult),
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    /// Output the cli result to the terminal
    Console,

    /// Output the cli result to a json file
    Json,
}

impl ValueEnum for OutputFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[OutputFormat::Console, OutputFormat::Json]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            OutputFormat::Console => {
                PossibleValue::new("console").help("Output the cli result to the terminal")
            }
            OutputFormat::Json => {
                PossibleValue::new("json").help("Output the cli result to a json file")
            }
        })
    }
}

fn format_output<T: Serialize>(
    input: T,
    format: OutputFormat,
    output_file: String,
) -> anyhow::Result<()> {
    match format {
        OutputFormat::Console => println!("{}", serde_json::to_string_pretty(&input)?),
        OutputFormat::Json => {
            serde_json::to_writer_pretty(File::create(format!("{output_file}.json"))?, &input)?;
            println!("Ok")
        }
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
        Command::Gas(cmd) => gas::parse(&execution_context, cmd).map(CliResult::GasNamespace),
        Command::Utils(_) => todo!(),
    }?;

    format_output(res, cli.out, cli.file)
}
