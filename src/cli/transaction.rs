use crate::context::CommandExecutionContext;

use super::common::NoArgs;
use clap::{command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command()]
pub struct TransactionCommand {
    #[arg(long)]
    hash: Option<String>,

    #[command(subcommand)]
    command: TransactionSubCommand,
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum TransactionSubCommand {
    Get(NoArgs),
}

pub fn parse(
    context: &CommandExecutionContext,
    sub_command: TransactionCommand,
) -> Result<(), anyhow::Error> {
    let TransactionCommand { hash, command } = sub_command;

    let res = match command {
        TransactionSubCommand::Get(_) => todo!(),
    };

    Ok(())
}
