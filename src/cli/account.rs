use crate::{cmd, context::CommandExecutionContext};

use super::common::NoArgs;
use clap::{command, Parser, Subcommand};
use ethers::types::{Address, NameOrAddress};

#[derive(Subcommand, Debug)]
#[command()]
pub enum AccountCommand {
    Balance(NoArgs),
}

#[derive(Parser, Debug)]
#[command()]
pub struct AccountSubCommand {
    #[arg(long, exclusive = true)]
    address: Option<String>,

    #[arg(long, exclusive = true)]
    ens: Option<String>,

    #[command(subcommand)]
    command: AccountCommand,
}

pub fn parse(
    context: &CommandExecutionContext,
    sub_command: AccountSubCommand,
) -> Result<(), anyhow::Error> {
    let AccountSubCommand {
        address,
        ens,
        command,
    } = sub_command;

    let account_id: NameOrAddress = if let Some(address) = address {
        NameOrAddress::Address(address.parse::<Address>()?)
    } else {
        // TODO: remove unwrap call
        NameOrAddress::Name(ens.unwrap())
    };

    match command {
        AccountCommand::Balance(_) => {
            context.execute(cmd::account::get_balance(context, account_id, None))?
        }
    };

    Ok(())
}
