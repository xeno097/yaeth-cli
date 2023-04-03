use crate::{cmd, context::CommandExecutionContext};

use super::common::{BlockTag, GetBlockById, NoArgs};
use clap::{command, Args, Parser, Subcommand};
use ethers::types::{Address, Bytes, NameOrAddress, H256, U256};

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

#[derive(Subcommand, Debug)]
#[command()]
pub enum AccountCommand {
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

#[derive(Args, Debug)]
pub struct GetStorageAtArgs {
    #[arg(short, long)]
    slot: String,
}

impl TryFrom<GetStorageAtArgs> for H256 {
    type Error = anyhow::Error;

    fn try_from(value: GetStorageAtArgs) -> Result<Self, Self::Error> {
        value
            .slot
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid hash format"))
    }
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

#[derive(Debug)]
pub enum BlockNamespaceResult {
    Bytecode(Bytes),
    Number(U256),
    Hash(H256),
}

impl From<U256> for BlockNamespaceResult {
    fn from(value: U256) -> Self {
        Self::Number(value)
    }
}

impl From<Bytes> for BlockNamespaceResult {
    fn from(value: Bytes) -> Self {
        Self::Bytecode(value)
    }
}

impl From<H256> for BlockNamespaceResult {
    fn from(value: H256) -> Self {
        Self::Hash(value)
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

    let res: BlockNamespaceResult = match command {
        AccountCommand::Balance(_) => context
            .execute(cmd::account::get_balance(
                context,
                account_id.into(),
                block_id.into(),
            ))?
            .into(),
        AccountCommand::Code(_) => context
            .execute(cmd::account::get_code(
                context,
                account_id.into(),
                block_id.into(),
            ))?
            .into(),
        AccountCommand::TransactionCount(_) => context
            .execute(cmd::account::get_transaction_count(
                context,
                account_id.into(),
                block_id.into(),
            ))?
            .into(),
        AccountCommand::Nonce(_) => context
            .execute(cmd::account::get_nonce(context, account_id.into()))?
            .into(),
        AccountCommand::StorageAt(storage_at_args) => context
            .execute(cmd::account::get_storage_at(
                context,
                account_id.into(),
                storage_at_args.try_into()?,
                block_id.into(),
            ))?
            .into(),
    };

    println!("{:#?}", res);

    Ok(())
}
