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

use super::common::{GetBlockArgs, NoArgs};
use clap::{arg, command, Args, Parser, Subcommand};
use ethers::{
    abi::Address,
    types::{Bytes, Transaction, TransactionReceipt, TransactionRequest, H256, U256, U64},
};
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
    get_block_by_id: GetBlockArgs,

    // TODO: reimplement the required constraint if any of the block ids field is set
    /// Index of the transaction in the block
    #[arg(long, value_name = "TRANSACTION_INDEX")]
    index: Option<u64>,
}

#[derive(Args, Debug)]
pub struct SendTransactionArgs {
    // Raw tx args
    /// Rlp encoded transaction data
    #[arg(long,conflicts_with_all(["from", "address", "ens","gas", "gas_price", "value", "data", "chain_id"]))]
    raw: Option<Bytes>,

    // Typed Tx args
    #[clap(flatten)]
    typed_tx: Option<TypedTransactionArgs>,

    // Config
    /// Wait for the transaction receipt
    #[arg(long)]
    wait: Option<bool>,
}

#[derive(Args, Debug)]
struct TypedTransactionArgs {
    #[arg(long)]
    from: Option<Address>,

    #[arg(long, conflicts_with = "ens")]
    address: Option<Address>,

    #[arg(long)]
    ens: Option<String>,

    #[arg(long)]
    gas: Option<U256>,

    #[arg(long)]
    gas_price: Option<U256>,

    /// Amount of Eth to send
    #[arg(long)]
    value: Option<U256>,

    #[arg(long)]
    data: Option<Bytes>,

    #[arg(long)]
    nonce: Option<U256>,

    #[arg(long)]
    chain_id: Option<U64>,
}

#[derive(Error, Debug)]
pub enum TypedTransactionParserError {
    #[error("Provided both ens and address")]
    ConflictingTransactionReceiver,
}

impl TryFrom<TypedTransactionArgs> for TransactionRequest {
    type Error = TypedTransactionParserError;

    fn try_from(value: TypedTransactionArgs) -> Result<Self, Self::Error> {
        let TypedTransactionArgs {
            from,
            address,
            ens,
            gas,
            gas_price,
            value,
            data,
            nonce,
            chain_id,
        } = value;

        let mut tx = TransactionRequest::new();

        if ens.is_some() && address.is_some() {
            return Err(Self::Error::ConflictingTransactionReceiver);
        }

        if let Some(from) = from {
            tx = tx.from(from)
        }

        if let Some(address) = address {
            tx = tx.to(address)
        }

        if let Some(ens) = ens {
            tx = tx.to(ens)
        }

        if let Some(gas) = gas {
            tx = tx.gas(gas)
        }

        if let Some(gas_price) = gas_price {
            tx = tx.gas_price(gas_price)
        }

        if let Some(value) = value {
            tx = tx.value(value)
        }

        if let Some(data) = data {
            tx = tx.data(data)
        }

        if let Some(nonce) = nonce {
            tx = tx.nonce(nonce)
        }

        if let Some(chain_id) = chain_id {
            tx = tx.chain_id(chain_id)
        }

        Ok(tx)
    }
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

impl TryFrom<GetTransactionArgs> for GetTransaction {
    type Error = anyhow::Error;

    fn try_from(value: GetTransactionArgs) -> Result<Self, Self::Error> {
        let GetTransactionArgs {
            get_block_by_id,
            index,
        } = value;

        if let Some(idx) = index {
            return Ok(Self::BlockIdAndIdx(
                get_block_by_id.try_into()?,
                idx as usize,
            ));
        }

        Err(anyhow::anyhow!(
            "Not provided enough identifiers for a transaction"
        ))
    }
}

#[derive(Args, Debug)]
pub struct SimulateTransactionArgs {
    #[clap(flatten)]
    typed_tx: TypedTransactionArgs,

    #[clap(flatten)]
    get_block_by_id: GetBlockArgs,
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

#[derive(Debug)]
pub enum TransactionNamespaceResult {
    Transaction(Transaction),
    SentTransaction(SendTxResult),
    Receipt(TransactionReceipt),
    Call(Bytes),
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
        TransactionSubCommand::Send(send_transaction_args) => context
            .execute(cmd::transaction::send_transaction(
                context,
                send_transaction_args.try_into()?,
            ))
            .map(TransactionNamespaceResult::SentTransaction)?,
        TransactionSubCommand::Call(simulate_transaction_args) => context
            .execute(cmd::transaction::call(
                context,
                simulate_transaction_args.try_into()?,
            ))
            .map(TransactionNamespaceResult::Call)?,
    };

    println!("{:#?}", res);

    Ok(())
}
