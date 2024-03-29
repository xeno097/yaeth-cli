use crate::{cmd, context::CommandExecutionContext};

use super::common::{GetAccountArgs, GetBlockByIdArgs, NoArgs};
use clap::{command, Args, Parser, Subcommand};
use ethers::types::{Bytes, H256, U256};
use serde::Serialize;

#[derive(Parser, Debug)]
#[command()]
pub struct AccountCommand {
    #[clap(flatten)]
    get_account_by_id: GetAccountArgs,

    #[clap(flatten)]
    get_block_by_id: GetBlockByIdArgs,

    #[command(subcommand)]
    command: AccountSubCommand,
}

#[derive(Args, Debug)]
pub struct GetStorageAtArgs {
    /// The storage slot where the target data is stored
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountNamespaceResult {
    Bytecode(Bytes),
    Number(U256),
    Hash(H256),
}

pub fn parse(
    context: &CommandExecutionContext,
    sub_command: AccountCommand,
) -> Result<AccountNamespaceResult, anyhow::Error> {
    let AccountCommand {
        get_account_by_id,
        get_block_by_id,
        command,
    } = sub_command;

    let account_id = get_account_by_id.try_into()?;

    let block_id = get_block_by_id.try_into().ok();

    let node_provider = context.node_provider();

    let res: AccountNamespaceResult = match command {
        AccountSubCommand::Balance(_) => context
            .execute(cmd::account::get_balance(
                node_provider,
                account_id,
                block_id,
            ))
            .map(AccountNamespaceResult::Number),
        AccountSubCommand::Code(_) => context
            .execute(cmd::account::get_code(node_provider, account_id, block_id))
            .map(AccountNamespaceResult::Bytecode),
        AccountSubCommand::TransactionCount(_) => context
            .execute(cmd::account::get_transaction_count(
                node_provider,
                account_id,
                block_id,
            ))
            .map(AccountNamespaceResult::Number),
        AccountSubCommand::Nonce(_) => context
            .execute(cmd::account::get_nonce(node_provider, account_id))
            .map(AccountNamespaceResult::Number),
        AccountSubCommand::StorageAt(GetStorageAtArgs { slot }) => context
            .execute(cmd::account::get_storage_at(
                node_provider,
                account_id,
                slot,
                block_id,
            ))
            .map(AccountNamespaceResult::Hash),
    }?;

    Ok(res)
}
