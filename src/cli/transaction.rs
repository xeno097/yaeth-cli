use crate::{
    cmd::{
        self,
        transaction::{
            GetTransaction, SendTransactionOptions, SendTxResult, SimulateTransactionOptions,
            TransactionKind,
        },
    },
    context::CommandExecutionContext,
};

use super::common::{
    parse_not_found, BlockIdParserError, GetBlockByIdArgs, NoArgs, TypedTransactionArgs,
    TypedTransactionParserError, GET_BLOCK_BY_ID_ARG_GROUP_NAME, TX_ARGS_FIELD_NAMES,
};
use clap::{arg, command, Args, Parser, Subcommand};
use ethers::types::{Bytes, Transaction, TransactionReceipt, H256};
use serde::Serialize;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command()]
pub struct TransactionCommand {
    /// Transaction hash. Required if using the receipt subcommand or get without options
    #[arg(long, value_name = "TRANSACTION_HASH")]
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

    /// Sends a transaction
    Send(SendTransactionArgs),

    /// Simulates a transaction without using any gas
    Call(SimulateTransactionArgs),
}

#[derive(Args, Debug)]
pub struct GetTransactionArgs {
    #[clap(flatten)]
    get_block_by_id: GetBlockByIdArgs,

    /// Index of the transaction in the block
    #[arg(long, value_name = "TRANSACTION_INDEX", requires = GET_BLOCK_BY_ID_ARG_GROUP_NAME)]
    index: Option<u64>,
}

#[derive(Args, Debug)]
pub struct SendTransactionArgs {
    // Raw tx args
    /// Rlp encoded transaction data
    #[arg(long,conflicts_with_all = TX_ARGS_FIELD_NAMES)]
    raw: Option<Bytes>,

    // Typed Tx args
    #[clap(flatten)]
    typed_tx: Option<TypedTransactionArgs>,

    // Config
    /// Wait for the transaction receipt
    #[arg(long)]
    wait: Option<bool>,
}

#[derive(Error, Debug)]
pub enum SendTransactionParserError {
    #[error("Specified raw transaction and typed transaction data.")]
    ConflictingTxData,

    #[error("{0}")]
    InvalidTypedTx(TypedTransactionParserError),

    #[error("Missing transaction data. Either a raw or typed transaction must be provided.")]
    MissingTxData,
}

impl TryFrom<SendTransactionArgs> for SendTransactionOptions {
    type Error = SendTransactionParserError;

    fn try_from(value: SendTransactionArgs) -> Result<Self, Self::Error> {
        let SendTransactionArgs {
            raw,
            typed_tx,
            wait,
        } = value;

        if raw.is_some() && typed_tx.is_some() {
            return Err(Self::Error::ConflictingTxData);
        }

        if let Some(raw) = raw {
            return Ok(Self::new(TransactionKind::RawTransaction(raw), wait));
        }

        if let Some(typed_tx) = typed_tx {
            return Ok(Self::new(
                TransactionKind::TypedTransaction(
                    typed_tx.try_into().map_err(Self::Error::InvalidTypedTx)?,
                ),
                wait,
            ));
        }

        Err(Self::Error::MissingTxData)
    }
}

#[derive(Error, Debug)]
pub enum GetTransactionParserError {
    #[error("{0}")]
    InvalidBlockId(BlockIdParserError),

    #[error("Missing transaction index.")]
    MissingIndex,
}

impl TryFrom<GetTransactionArgs> for GetTransaction {
    type Error = GetTransactionParserError;

    fn try_from(value: GetTransactionArgs) -> Result<Self, Self::Error> {
        let GetTransactionArgs {
            get_block_by_id,
            index,
        } = value;

        let idx = index.ok_or(Self::Error::MissingIndex)?;

        Ok(Self::BlockIdAndIdx(
            get_block_by_id
                .try_into()
                .map_err(Self::Error::InvalidBlockId)?,
            idx as usize,
        ))
    }
}

#[derive(Args, Debug)]
pub struct SimulateTransactionArgs {
    #[clap(flatten)]
    typed_tx: TypedTransactionArgs,

    #[clap(flatten)]
    get_block_by_id: GetBlockByIdArgs,
}

#[derive(Error, Debug)]
pub enum SimulateTransactionParserError {
    #[error("{0}")]
    TypedTxParserError(TypedTransactionParserError),
}

impl TryFrom<SimulateTransactionArgs> for SimulateTransactionOptions {
    type Error = SimulateTransactionParserError;

    fn try_from(value: SimulateTransactionArgs) -> Result<Self, Self::Error> {
        let SimulateTransactionArgs {
            typed_tx,
            get_block_by_id,
        } = value;

        Ok(SimulateTransactionOptions::new(
            typed_tx
                .try_into()
                .map_err(Self::Error::TypedTxParserError)?,
            get_block_by_id.try_into().ok(),
        ))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionNamespaceResult {
    Transaction(Transaction),
    SentTransaction(SendTxResult),
    Receipt(TransactionReceipt),
    Call(Bytes),
    #[serde(serialize_with = "parse_not_found", rename = "transaction")]
    NotFound(),
}

pub fn parse(
    context: &CommandExecutionContext,
    sub_command: TransactionCommand,
) -> Result<TransactionNamespaceResult, anyhow::Error> {
    let TransactionCommand { hash, command } = sub_command;

    let node_provider = context.node_provider();

    let res: TransactionNamespaceResult = match command {
        TransactionSubCommand::Get(get_transaction_args) => context
            .execute(cmd::transaction::get_transaction(
                node_provider,
                hash.map(GetTransaction::TransactionHash)
                    .map_or_else(|| get_transaction_args.try_into(), Ok)?,
            ))?
            .map_or_else(
                TransactionNamespaceResult::NotFound,
                TransactionNamespaceResult::Transaction,
            ),
        TransactionSubCommand::Receipt(_) => context
            .execute(cmd::transaction::get_transaction_receipt(
                node_provider,
                hash.ok_or(anyhow::anyhow!(
                    "Missing required argument transaction hash"
                ))?,
            ))?
            .map_or_else(
                TransactionNamespaceResult::NotFound,
                TransactionNamespaceResult::Receipt,
            ),
        TransactionSubCommand::Send(send_transaction_args) => context
            .execute(cmd::transaction::send_transaction(
                node_provider,
                send_transaction_args.try_into()?,
            ))
            .map(TransactionNamespaceResult::SentTransaction)?,
        TransactionSubCommand::Call(simulate_transaction_args) => context
            .execute(cmd::transaction::call(
                node_provider,
                simulate_transaction_args.try_into()?,
            ))
            .map(TransactionNamespaceResult::Call)?,
    };

    Ok(res)
}
