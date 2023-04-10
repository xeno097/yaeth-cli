use crate::{cmd, context::CommandExecutionContext};

use super::common::{GetBlockArgs, NoArgs};
use clap::{command, Args, Parser, Subcommand};
use ethers::types::{Bytes, NameOrAddress, H160, H256, U256};

#[derive(Parser, Debug)]
#[command()]
pub struct AccountCommand {
    #[clap(flatten)]
    get_account_by_id: GetAccountArgs,

    #[clap(flatten)]
    get_block_by_id: GetBlockArgs,

    #[command(subcommand)]
    command: AccountSubCommand,
}

#[derive(Args, Debug)]
pub struct GetAccountArgs {
    #[arg(long, conflicts_with = "ens", required_unless_present = "ens")]
    address: Option<H160>,

    #[arg(long)]
    ens: Option<String>,
}

impl TryFrom<GetAccountArgs> for NameOrAddress {
    type Error = anyhow::Error;

    fn try_from(GetAccountArgs { address, ens }: GetAccountArgs) -> Result<Self, Self::Error> {
        // Sanity check
        if address.is_some() && ens.is_some() {
            return Err(anyhow::anyhow!("Provided multiple address identifiers"));
        }

        let ret = if let Some(address) = address {
            NameOrAddress::Address(address)
        } else {
            NameOrAddress::Name(ens.unwrap())
        };

        Ok(ret)
    }
}

#[derive(Args, Debug)]
pub struct GetStorageAtArgs {
    #[arg(short, long)]
    slot: H256,
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum AccountSubCommand {
    /// Retrieves the account balance in the specified block (defaults to latest)
    Balance(NoArgs),

    /// Retrieves the account bytecode in the specified block (defaults to latest)
    Code(NoArgs),

    /// Retrieves the account transaction count in the specified block (defaults to latest)
    TransactionCount(NoArgs),

    /// Retrieves the account nonce
    Nonce(NoArgs),

    /// Retrieves the value stored in the specified storage slot and block (defaults to latest)
    StorageAt(GetStorageAtArgs),
}

#[derive(Debug)]
pub enum BlockNamespaceResult {
    Bytecode(Bytes),
    Number(U256),
    Hash(H256),
}

pub fn parse(
    context: &CommandExecutionContext,
    sub_command: AccountCommand,
) -> Result<(), anyhow::Error> {
    let AccountCommand {
        get_account_by_id,
        get_block_by_id,
        command,
    } = sub_command;

    let account_id = get_account_by_id.try_into()?;

    let block_id = get_block_by_id.try_into().ok();

    let res: BlockNamespaceResult = match command {
        AccountSubCommand::Balance(_) => context
            .execute(cmd::account::get_balance(context, account_id, block_id))
            .map(BlockNamespaceResult::Number)?,
        AccountSubCommand::Code(_) => context
            .execute(cmd::account::get_code(context, account_id, block_id))
            .map(BlockNamespaceResult::Bytecode)?,
        AccountSubCommand::TransactionCount(_) => context
            .execute(cmd::account::get_transaction_count(
                context, account_id, block_id,
            ))
            .map(BlockNamespaceResult::Number)?,
        AccountSubCommand::Nonce(_) => context
            .execute(cmd::account::get_nonce(context, account_id))
            .map(BlockNamespaceResult::Number)?,
        AccountSubCommand::StorageAt(GetStorageAtArgs { slot }) => context
            .execute(cmd::account::get_storage_at(
                context, account_id, slot, block_id,
            ))
            .map(BlockNamespaceResult::Hash)?,
    };

    println!("{:#?}", res);

    Ok(())
}
