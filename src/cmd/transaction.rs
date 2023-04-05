use anyhow::Ok;
use ethers::{
    providers::{Http, Middleware, PendingTransaction},
    types::{BlockId, Bytes, Transaction, TransactionReceipt, TransactionRequest, H256},
};

use crate::context::CommandExecutionContext;

pub enum GetTransaction {
    TransactionHash(H256),
    BlockIdAndIdx(BlockId, usize),
}

pub async fn get_transaction(
    context: &CommandExecutionContext,
    get_by: GetTransaction,
) -> anyhow::Result<Option<Transaction>> {
    match get_by {
        GetTransaction::TransactionHash(hash) => get_transaction_by_hash(context, hash).await,
        GetTransaction::BlockIdAndIdx(block_id, idx) => {
            get_transaction_block_id_and_idx(context, block_id, idx).await
        }
    }
}

// eth_getTransactionByHash
async fn get_transaction_by_hash(
    context: &CommandExecutionContext,
    hash: H256,
) -> anyhow::Result<Option<Transaction>> {
    let tx = context.node_provider().get_transaction(hash).await?;

    Ok(tx)
}

// eth_getTransactionByBlockHashAndIndex || eth_getTransactionByBlockNumberAndIndex
async fn get_transaction_block_id_and_idx(
    context: &CommandExecutionContext,
    block_id: BlockId,
    idx: usize,
) -> anyhow::Result<Option<Transaction>> {
    let block = context.node_provider().get_block_with_txs(block_id).await?;

    if let Some(block) = block {
        let tx = block.transactions.get(idx).cloned();

        return Ok(tx);
    }

    Ok(None)
}

// eth_getTransactionReceipt
pub async fn get_transaction_receipt(
    context: &CommandExecutionContext,
    hash: H256,
) -> anyhow::Result<Option<TransactionReceipt>> {
    let receipt = context
        .node_provider()
        .get_transaction_receipt(hash)
        .await?;

    Ok(receipt)
}

pub enum TransactionKind {
    RawTransaction(Bytes),
    TypedTransaction(TransactionRequest),
}

pub struct SendTransactionOptions {
    tx_data: TransactionKind,
    wait: bool,
}

impl SendTransactionOptions {
    pub fn new(data: TransactionKind, wait: Option<bool>) -> Self {
        Self {
            tx_data: data,
            wait: wait.unwrap_or(false),
        }
    }
}

#[derive(Debug)]
pub enum TxResult {
    PendingTransaction(H256),
    Receipt(Option<TransactionReceipt>),
}

pub async fn send_transaction(
    context: &CommandExecutionContext,
    tx_data: SendTransactionOptions,
) -> anyhow::Result<TxResult> {
    let SendTransactionOptions { tx_data, wait } = tx_data;

    let pending_tx = match tx_data {
        TransactionKind::RawTransaction(raw_tx) => send_raw_transaction(context, raw_tx).await?,
        TransactionKind::TypedTransaction(tx) => send_typed_transaction(context, tx).await?,
    };

    let res = if wait {
        TxResult::Receipt(pending_tx.await?)
    } else {
        TxResult::PendingTransaction(pending_tx.tx_hash())
    };

    Ok(res)
}

// eth_sendRawTransaction
async fn send_raw_transaction(
    context: &CommandExecutionContext,
    encoded_tx: Bytes,
) -> anyhow::Result<PendingTransaction<Http>> {
    let receipt = context
        .node_provider()
        .send_raw_transaction(encoded_tx)
        .await?;

    Ok(receipt)
}

async fn send_typed_transaction(
    context: &CommandExecutionContext,
    tx: TransactionRequest,
) -> anyhow::Result<PendingTransaction<Http>> {
    let receipt = context.node_provider().send_transaction(tx, None).await?;

    Ok(receipt)
}
