use ethers::{
    providers::Middleware,
    types::{BlockId, BlockNumber, Bytes, NameOrAddress, H256, U256},
};

use crate::context::CommandExecutionContext;

pub async fn get_balance(
    context: &CommandExecutionContext,
    account_id: NameOrAddress,
    block_id: BlockId,
) -> anyhow::Result<U256> {
    let balance = context
        .node_provider()
        .get_balance(account_id, Some(block_id))
        .await?;

    Ok(balance)
}

pub async fn get_code(
    context: &CommandExecutionContext,
    account_id: NameOrAddress,
    block_id: BlockId,
) -> anyhow::Result<Bytes> {
    let bytecode = context
        .node_provider()
        .get_code(account_id, Some(block_id))
        .await?;

    Ok(bytecode)
}

pub async fn get_transaction_count(
    context: &CommandExecutionContext,
    account_id: NameOrAddress,
    block_id: BlockId,
) -> anyhow::Result<U256> {
    let res = context
        .node_provider()
        .get_transaction_count(account_id, Some(block_id))
        .await?;

    Ok(res)
}

pub async fn get_nonce(
    context: &CommandExecutionContext,
    account_id: NameOrAddress,
) -> anyhow::Result<U256> {
    get_transaction_count(context, account_id, BlockId::Number(BlockNumber::Latest)).await
}

pub async fn get_storage_at(
    context: &CommandExecutionContext,
    account_id: NameOrAddress,
    slot: H256,
    block_id: BlockId,
) -> anyhow::Result<H256> {
    let res = context
        .node_provider()
        .get_storage_at(account_id, slot, Some(block_id))
        .await?;

    Ok(res)
}
