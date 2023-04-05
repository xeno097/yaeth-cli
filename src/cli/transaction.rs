use crate::{
    cli::common::GetBlockById,
    cmd::{
        self,
        transaction::{GetTransaction, SendTransactionOptions, TransactionKind, TxResult},
    },
    context::CommandExecutionContext,
};

use super::common::{BlockTag, NoArgs};
use clap::{arg, command, Args, Parser, Subcommand};
use ethers::{
    abi::Address,
    types::{Bytes, Transaction, TransactionReceipt, TransactionRequest, H256, U256, U64},
};

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

    /// Sends a transaction
    Send(SendTransactionArgs),
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

#[derive(Args, Debug)]
pub struct SendTransactionArgs {
    // Raw tx args
    #[arg(long,conflicts_with_all(["from", "address", "ens","gas", "gas_price", "value", "data", "chain_id"]))]
    raw: Option<Bytes>,

    // Typed Tx args
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

    #[arg(long)]
    value: Option<U256>,

    #[arg(long)]
    data: Option<Bytes>,

    #[arg(long)]
    nonce: Option<U256>,

    #[arg(long)]
    chain_id: Option<U64>,

    // Config
    #[arg(long)]
    wait: Option<bool>,
}

impl TryFrom<SendTransactionArgs> for SendTransactionOptions {
    type Error = anyhow::Error;

    fn try_from(value: SendTransactionArgs) -> Result<Self, Self::Error> {
        let SendTransactionArgs {
            raw,
            from,
            address,
            ens,
            gas,
            gas_price,
            value,
            data,
            nonce,
            chain_id,
            wait,
        } = value;

        // TODO: check that only raw is set and not any other field exlcuindg wait

        if let Some(raw) = raw {
            return Ok(Self::new(TransactionKind::RawTransaction(raw), wait));
        }

        let mut tx = TransactionRequest::new();

        if ens.is_some() && address.is_some() {
            return Err(anyhow::anyhow!("ens and address are conflicting arguments"));
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

        Ok(Self::new(TransactionKind::TypedTransaction(tx), wait))
    }
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
    SentTransaction(TxResult),
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
        TransactionSubCommand::Send(send_transaction_args) => context
            .execute(cmd::transaction::send_transaction(
                context,
                send_transaction_args.try_into()?,
            ))
            .map(TransactionNamespaceResult::SentTransaction)?,
    };

    println!("{:#?}", res);

    Ok(())
}
