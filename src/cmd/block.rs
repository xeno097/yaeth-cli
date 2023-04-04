use crate::context::CommandExecutionContext;
use anyhow::Ok;
use ethers::{
    providers::Middleware,
    types::{Block, BlockId, BlockNumber, Transaction, TransactionReceipt, H256, U256, U64},
};

#[derive(Debug)]
pub enum BlockDataResult<T> {
    Data(T),
    NotFound(),
}

pub enum BlockKind {
    RawBlock(Block<H256>),
    BlockWithTransaction(Block<Transaction>),
}

impl From<BlockDataResult<Block<H256>>> for BlockDataResult<BlockKind> {
    fn from(value: BlockDataResult<Block<H256>>) -> Self {
        match value {
            BlockDataResult::Data(block) => BlockDataResult::Data(BlockKind::RawBlock(block)),
            _ => BlockDataResult::NotFound(),
        }
    }
}

impl From<BlockDataResult<Block<Transaction>>> for BlockDataResult<BlockKind> {
    fn from(value: BlockDataResult<Block<Transaction>>) -> Self {
        match value {
            BlockDataResult::Data(block) => {
                BlockDataResult::Data(BlockKind::BlockWithTransaction(block))
            }
            _ => BlockDataResult::NotFound(),
        }
    }
}

// eth_getBlockByHash || eth_getBlockByNumber
pub async fn get_block(
    context: &CommandExecutionContext,
    block_id: BlockId,
    include_tx: bool,
) -> Result<BlockDataResult<BlockKind>, anyhow::Error> {
    if include_tx {
        get_raw_block(context, block_id).await.map(|res| res.into())
    } else {
        get_block_with_txs(context, block_id)
            .await
            .map(|res| res.into())
    }
}

async fn get_raw_block(
    context: &CommandExecutionContext,
    block_id: BlockId,
) -> Result<BlockDataResult<Block<H256>>, anyhow::Error> {
    let block = context.node_provider().get_block(block_id).await?;

    if let Some(block) = block {
        return Ok(BlockDataResult::Data(block));
    }

    Ok(BlockDataResult::NotFound())
}

async fn get_block_with_txs(
    context: &CommandExecutionContext,
    block_id: BlockId,
) -> Result<BlockDataResult<Block<Transaction>>, anyhow::Error> {
    let block = context.node_provider().get_block_with_txs(block_id).await?;

    if let Some(block) = block {
        return Ok(BlockDataResult::Data(block));
    }

    Ok(BlockDataResult::NotFound())
}

// eth_blockNumber
pub async fn get_block_number(context: &CommandExecutionContext) -> Result<U64, anyhow::Error> {
    let block_number = context.node_provider().get_block_number().await?;

    Ok(block_number)
}

// eth_getBlockTransactionCountByHash || eth_getBlockTransactionCountByNumber
pub async fn get_transaction_count(
    context: &CommandExecutionContext,
    block_id: BlockId,
) -> Result<BlockDataResult<U256>, anyhow::Error> {
    match get_raw_block(context, block_id).await? {
        BlockDataResult::Data(block) => {
            Ok(BlockDataResult::Data(U256::from(block.transactions.len())))
        }
        BlockDataResult::NotFound() => Ok(BlockDataResult::NotFound()),
    }
}

// eth_getUncleCountByBlockHash || eth_getUncleCountByBlockNumber
pub async fn get_uncle_block_count(
    context: &CommandExecutionContext,
    block_id: BlockId,
) -> Result<U256, anyhow::Error> {
    let count = context.node_provider().get_uncle_count(block_id).await?;

    Ok(count)
}

// eth_getBlockReceipts
pub async fn get_block_receipts(
    context: &CommandExecutionContext,
    block_id: BlockId,
) -> Result<BlockDataResult<Vec<TransactionReceipt>>, anyhow::Error> {
    let block_id: BlockNumber = match block_id {
        BlockId::Hash(hash) => match get_raw_block(context, hash.into()).await? {
            BlockDataResult::Data(block) => {
                BlockNumber::from(block.number.ok_or(anyhow::anyhow!(
                    "Block number not found for the block with the provided block hash"
                ))?)
            }
            _ => return Ok(BlockDataResult::NotFound()),
        },
        BlockId::Number(num) => num,
    };

    let receipts = context.node_provider().get_block_receipts(block_id).await?;

    Ok(BlockDataResult::Data(receipts))
}
