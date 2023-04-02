use crate::{cmd, context::CommandExecutionContext};

use super::common::{BlockTag, GetBlockById, NoArgs};
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
    #[arg(long)]
    address: Option<String>,

    #[arg(long)]
    ens: Option<String>,

    #[arg(long)]
    hash: Option<String>,

    #[arg(long)]
    number: Option<u64>,

    #[arg(long)]
    tag: Option<BlockTag>,

    #[command(subcommand)]
    command: AccountCommand,
}

struct GetAddressById(NameOrAddress);

impl GetAddressById {
    pub fn new(address: Option<String>, ens: Option<String>) -> anyhow::Result<GetAddressById> {
        // Sanity check
        if address.is_some() && ens.is_some() {
            return Err(anyhow::anyhow!("Provided multiple address identifiers"));
        }

        let ret = if let Some(address) = address {
            NameOrAddress::Address(address.parse::<Address>()?)
        } else {
            NameOrAddress::Name(ens.unwrap())
        };

        Ok(GetAddressById(ret))
    }
}

impl From<GetAddressById> for NameOrAddress {
    fn from(value: GetAddressById) -> Self {
        value.0
    }
}

pub fn parse(
    context: &CommandExecutionContext,
    sub_command: AccountSubCommand,
) -> Result<(), anyhow::Error> {
    let AccountSubCommand {
        address,
        ens,
        hash,
        number,
        tag,
        command,
    } = sub_command;

    let account_id = GetAddressById::new(address, ens)?;

    let block_id = GetBlockById::new(hash, number, tag)?;

    let res = match command {
        AccountCommand::Balance(_) => context.execute(cmd::account::get_balance(
            context,
            account_id.into(),
            block_id.into(),
        ))?,
    };

    println!("{:#?}", res);

    Ok(())
}
