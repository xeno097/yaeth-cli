use crate::{
    cli::common::GetBlockById,
    cmd::{self, transaction::GetTransaction},
    context::CommandExecutionContext,
};

use super::common::{BlockTag, NoArgs};
use clap::{arg, command, Args, Parser, Subcommand};
use ethers::types::{Transaction, TransactionReceipt, H256};

#[derive(Parser, Debug)]
#[command()]
pub struct TransactionCommand {
    #[arg(long)]
    hash: Option<H256>,

    #[command(subcommand)]
    command: TransactionSubCommand,
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum TransactionSubCommand {
    /// Gets a transaction by the provided identifier
    Get(GetTransactionArgs),

    /// Gets a transaction receipt by transaction hash
    Receipt(NoArgs),
}

#[derive(Args, Debug)]
pub struct GetTransactionArgs {
    #[arg(long, conflicts_with_all(["number","tag"]), requires= "index")]
    hash: Option<String>,

    #[arg(long, conflicts_with_all(["hash","tag"]),requires= "index")]
    number: Option<u64>,

    #[arg(long, conflicts_with_all(["hash","number"]),requires= "index")]
    tag: Option<BlockTag>,

    #[arg(long)]
    index: Option<u64>,
}

impl TryFrom<GetTransactionArgs> for GetTransaction {
    type Error = anyhow::Error;

    fn try_from(value: GetTransactionArgs) -> Result<Self, Self::Error> {
        let GetTransactionArgs {
            hash,
            index,
            number,
            tag,
        } = value;

        let block_id = GetBlockById::new(hash, number, tag)?;

        if let Some(idx) = index {
            return Ok(Self::BlockIdAndIdx(block_id.into(), idx as usize));
        }

        Err(anyhow::anyhow!(
            "Not provided enough identifiers for a transaction"
        ))
    }
}

#[derive(Debug)]
pub enum TransactionNamespaceResult {
    Transaction(Transaction),
    Receipt(TransactionReceipt),
    NotFound(),
}

pub fn parse(
    context: &CommandExecutionContext,
    sub_command: TransactionCommand,
) -> Result<(), anyhow::Error> {
    let TransactionCommand { hash, command } = sub_command;

    let res: TransactionNamespaceResult = match command {
        TransactionSubCommand::Get(get_transaction_args) => {
            let tx_id = if let Some(hash) = hash {
                GetTransaction::TransactionHash(hash)
            } else {
                get_transaction_args.try_into()?
            };

            context
                .execute(cmd::transaction::get_transaction(context, tx_id))?
                .map_or_else(
                    TransactionNamespaceResult::NotFound,
                    TransactionNamespaceResult::Transaction,
                )
        }
        TransactionSubCommand::Receipt(_) => context
            .execute(cmd::transaction::get_transaction_receipt(
                context,
                hash.ok_or(anyhow::anyhow!("Missing required argument hash"))?,
            ))?
            .map_or_else(
                TransactionNamespaceResult::NotFound,
                TransactionNamespaceResult::Receipt,
            ),
    };

    println!("{:#?}", res);

    Ok(())
}
