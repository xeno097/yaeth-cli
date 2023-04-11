use crate::context::NodeProvider;
use anyhow::Ok;
use ethers::{
    providers::Middleware,
    types::{Block, BlockId, BlockNumber, Transaction, TransactionReceipt, H256, U256, U64},
};

#[derive(Debug)]
pub enum BlockKind {
    RawBlock(Block<H256>),
    BlockWithTransaction(Block<Transaction>),
}

// eth_getBlockByHash || eth_getBlockByNumber
pub async fn get_block(
    node_provider: &NodeProvider,
    block_id: BlockId,
    include_tx: bool,
) -> Result<Option<BlockKind>, anyhow::Error> {
    let res = if include_tx {
        get_block_with_txs(node_provider, block_id)
            .await?
            .map(BlockKind::BlockWithTransaction)
    } else {
        get_raw_block(node_provider, block_id)
            .await?
            .map(BlockKind::RawBlock)
    };

    Ok(res)
}

async fn get_raw_block(
    node_provider: &NodeProvider,
    block_id: BlockId,
) -> Result<Option<Block<H256>>, anyhow::Error> {
    let block = node_provider.get_block(block_id).await?;

    if let Some(block) = block {
        return Ok(Some(block));
    }

    Ok(None)
}

async fn get_block_with_txs(
    node_provider: &NodeProvider,
    block_id: BlockId,
) -> Result<Option<Block<Transaction>>, anyhow::Error> {
    let block = node_provider.get_block_with_txs(block_id).await?;

    if let Some(block) = block {
        return Ok(Some(block));
    }

    Ok(None)
}

// eth_blockNumber
pub async fn get_block_number(node_provider: &NodeProvider) -> Result<U64, anyhow::Error> {
    let block_number = node_provider.get_block_number().await?;

    Ok(block_number)
}

// eth_getBlockTransactionCountByHash || eth_getBlockTransactionCountByNumber
pub async fn get_transaction_count(
    node_provider: &NodeProvider,
    block_id: BlockId,
) -> Result<Option<U256>, anyhow::Error> {
    if let Some(block) = get_raw_block(node_provider, block_id).await? {
        return Ok(Some(U256::from(block.transactions.len())));
    }

    Ok(None)
}

// eth_getUncleCountByBlockHash || eth_getUncleCountByBlockNumber
pub async fn get_uncle_block_count(
    node_provider: &NodeProvider,
    block_id: BlockId,
) -> Result<U256, anyhow::Error> {
    let count = node_provider.get_uncle_count(block_id).await?;

    Ok(count)
}

// eth_getBlockReceipts
pub async fn get_block_receipts(
    node_provider: &NodeProvider,
    block_id: BlockId,
) -> Result<Option<Vec<TransactionReceipt>>, anyhow::Error> {
    let block_id: BlockNumber = match block_id {
        BlockId::Hash(hash) => match get_raw_block(node_provider, hash.into()).await? {
            Some(block) => BlockNumber::from(block.number.ok_or(anyhow::anyhow!(
                "Block number not found for the block with the provided block hash"
            ))?),
            None => return Ok(None),
        },
        BlockId::Number(num) => num,
    };

    let receipts = node_provider.get_block_receipts(block_id).await?;

    Ok(Some(receipts))
}
