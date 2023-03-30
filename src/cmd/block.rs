use crate::{
    cli::block::{GetBlockArgs, GetBlockTransactionCountArgs},
    context::CommandExecutionContext,
};
use ethers::{
    providers::Middleware,
    types::{BlockId, BlockNumber},
};

// eth_getBlockByHash || eth_getBlockByNumber
pub async fn get_block(
    context: &CommandExecutionContext,
    get_block_args: GetBlockArgs,
) -> Result<(), anyhow::Error> {
    let block_id: BlockId = get_block_args.try_into().unwrap();

    let block = context.node_provider().get_block(block_id).await?;

    println!("{:#?}", block);

    Ok(())
}

// eth_getBlockTransactionCountByHash || eth_getBlockTransactionCountByNumber
pub async fn get_transaction_count(
    context: &CommandExecutionContext,
    get_block_transaction_count: GetBlockTransactionCountArgs,
) -> Result<(), anyhow::Error> {
    let block_id: BlockId = get_block_transaction_count.try_into().unwrap();

    let block = context.node_provider().get_block(block_id).await?;

    let transaction_count = block.unwrap().transactions.len();

    println!("{:#?}", transaction_count);

    Ok(())
}

// eth_getUncleCountByBlockHash || eth_getUncleCountByBlockNumber
pub async fn get_uncle_block_count(
    context: &CommandExecutionContext,
    get_block_transaction_count: GetBlockTransactionCountArgs,
) -> Result<(), anyhow::Error> {
    let block_id: BlockId = get_block_transaction_count.try_into().unwrap();

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
    get_block_args: GetBlockTransactionCountArgs,
) -> Result<(), anyhow::Error> {
    let block_id: BlockId = get_block_args.try_into().unwrap();

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
