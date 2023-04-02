use crate::context::CommandExecutionContext;
use anyhow::Ok;
use ethers::{
    providers::Middleware,
    types::{Block, BlockId, BlockNumber, Transaction, TransactionReceipt, H256, U256},
};

#[derive(Debug)]
pub enum BlockNamespaceResult {
    Block(Block<H256>),
    BlockWithTransaction(Block<Transaction>),
    BlockNumber(u64),
    TransactionCount(U256),
    TransactionReceipts(Vec<TransactionReceipt>),
    UncleBlockCount(U256),
    NotFound(),
}

// eth_getBlockByHash || eth_getBlockByNumber
pub async fn get_block(
    context: &CommandExecutionContext,
    block_id: BlockId,
    include_tx: bool,
) -> Result<BlockNamespaceResult, anyhow::Error> {
    if include_tx {
        get_raw_block(context, block_id).await
    } else {
        get_block_with_txs(context, block_id).await
    }
}

async fn get_raw_block(
    context: &CommandExecutionContext,
    block_id: BlockId,
) -> Result<BlockNamespaceResult, anyhow::Error> {
    let block = context.node_provider().get_block(block_id).await?;

    if let Some(block) = block {
        return Ok(BlockNamespaceResult::Block(block));
    }

    Ok(BlockNamespaceResult::NotFound())
}

async fn get_block_with_txs(
    context: &CommandExecutionContext,
    block_id: BlockId,
) -> Result<BlockNamespaceResult, anyhow::Error> {
    let block = context.node_provider().get_block_with_txs(block_id).await?;

    if let Some(block) = block {
        return Ok(BlockNamespaceResult::BlockWithTransaction(block));
    }

    Ok(BlockNamespaceResult::NotFound())
}

// eth_blockNumber
pub async fn get_block_number(
    context: &CommandExecutionContext,
) -> Result<BlockNamespaceResult, anyhow::Error> {
    let block_number = context.node_provider().get_block_number().await?;

    Ok(BlockNamespaceResult::BlockNumber(block_number.as_u64()))
}

// eth_getBlockTransactionCountByHash || eth_getBlockTransactionCountByNumber
pub async fn get_transaction_count(
    context: &CommandExecutionContext,
    block_id: BlockId,
) -> Result<BlockNamespaceResult, anyhow::Error> {
    match get_raw_block(context, block_id).await? {
        BlockNamespaceResult::Block(block) => Ok(BlockNamespaceResult::TransactionCount(
            U256::from(block.transactions.len()),
        )),
        _ => Ok(BlockNamespaceResult::NotFound()),
    }
}

// eth_getUncleCountByBlockHash || eth_getUncleCountByBlockNumber
pub async fn get_uncle_block_count(
    context: &CommandExecutionContext,
    block_id: BlockId,
) -> Result<BlockNamespaceResult, anyhow::Error> {
    let count = context.node_provider().get_uncle_count(block_id).await?;

    Ok(BlockNamespaceResult::UncleBlockCount(count))
}

// eth_getBlockReceipts
pub async fn get_block_receipts(
    context: &CommandExecutionContext,
    block_id: BlockId,
) -> Result<BlockNamespaceResult, anyhow::Error> {
    let block_id: BlockNumber = match block_id {
        BlockId::Hash(hash) => match get_raw_block(context, hash.into()).await? {
            BlockNamespaceResult::Block(block) => {
                BlockNumber::from(block.number.ok_or(anyhow::anyhow!(
                    "Block number not found for the block with the provided block hash"
                ))?)
            }
            _ => return Ok(BlockNamespaceResult::NotFound()),
        },
        BlockId::Number(num) => num,
    };

    let receipts = context.node_provider().get_block_receipts(block_id).await?;

    Ok(BlockNamespaceResult::TransactionReceipts(receipts))
}
