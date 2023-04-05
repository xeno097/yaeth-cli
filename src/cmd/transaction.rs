use anyhow::Ok;
use ethers::{
    providers::Middleware,
    types::{BlockId, Transaction, TransactionReceipt, H256},
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
