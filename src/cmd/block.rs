use crate::context::CommandExecutionContext;
use anyhow::Ok;
use ethers::{
    providers::Middleware,
    types::{Block, BlockId, BlockNumber, Transaction, H256},
};

#[derive(Debug)]
pub enum GetBlockResult {
    Block(Block<H256>),
    BlockWithTransaction(Block<Transaction>),
    NotFound(),
}

// eth_getBlockByHash || eth_getBlockByNumber
pub async fn get_block(
    context: &CommandExecutionContext,
    block_id: BlockId,
    include_tx: bool,
) -> Result<GetBlockResult, anyhow::Error> {
    if include_tx {
        let block = context.node_provider().get_block_with_txs(block_id).await?;

        if let Some(block) = block {
            return Ok(GetBlockResult::BlockWithTransaction(block));
        }

        return Ok(GetBlockResult::NotFound());
    }

    let block = context.node_provider().get_block(block_id).await?;

    if let Some(block) = block {
        return Ok(GetBlockResult::Block(block));
    }

    Ok(GetBlockResult::NotFound())
}

// eth_getBlockTransactionCountByHash || eth_getBlockTransactionCountByNumber
pub async fn get_transaction_count(
    context: &CommandExecutionContext,
    block_id: BlockId,
) -> Result<(), anyhow::Error> {
    let block = context.node_provider().get_block(block_id).await?;

    let transaction_count = block.unwrap().transactions.len();

    println!("{:#?}", transaction_count);

    Ok(())
}

// eth_getUncleCountByBlockHash || eth_getUncleCountByBlockNumber
pub async fn get_uncle_block_count(
    context: &CommandExecutionContext,
    block_id: BlockId,
) -> Result<(), anyhow::Error> {
    let block = context.node_provider().get_uncle_count(block_id).await?;

    println!("{:#?}", block);

    Ok(())
}

// eth_blockNumber
pub async fn get_block_number(context: &CommandExecutionContext) -> Result<(), anyhow::Error> {
    let block_number = context.node_provider().get_block_number().await?;

    println!("{:#?}", block_number);

    Ok(())
}

// eth_getBlockReceipts
pub async fn get_block_receipts(
    context: &CommandExecutionContext,
    block_id: BlockId,
) -> Result<(), anyhow::Error> {
    let block_id: BlockNumber = match block_id {
        BlockId::Hash(hash) => {
            let block = context.node_provider().get_block(hash).await?;

            BlockNumber::from(block.unwrap().number.unwrap())
        }
        BlockId::Number(num) => num,
    };

    let block_number = context.node_provider().get_block_receipts(block_id).await?;

    println!("{:#?}", block_number);

    Ok(())
}
