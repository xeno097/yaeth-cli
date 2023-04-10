use ethers::{
    providers::Middleware,
    types::{BlockId, Bytes, NameOrAddress, H256, U256},
};

use crate::context::CommandExecutionContext;

// eth_getBalance
pub async fn get_balance(
    context: &CommandExecutionContext,
    account_id: NameOrAddress,
    block_id: Option<BlockId>,
) -> anyhow::Result<U256> {
    let balance = context
        .node_provider()
        .get_balance(account_id, block_id)
        .await?;

    Ok(balance)
}

// eth_getCode
pub async fn get_code(
    context: &CommandExecutionContext,
    account_id: NameOrAddress,
    block_id: Option<BlockId>,
) -> anyhow::Result<Bytes> {
    let bytecode = context
        .node_provider()
        .get_code(account_id, block_id)
        .await?;

    Ok(bytecode)
}

// eth_getTransactionCount
pub async fn get_transaction_count(
    context: &CommandExecutionContext,
    account_id: NameOrAddress,
    block_id: Option<BlockId>,
) -> anyhow::Result<U256> {
    let transaction_count = context
        .node_provider()
        .get_transaction_count(account_id, block_id)
        .await?;

    Ok(transaction_count)
}

pub async fn get_nonce(
    context: &CommandExecutionContext,
    account_id: NameOrAddress,
) -> anyhow::Result<U256> {
    get_transaction_count(context, account_id, None).await
}

// eth_getStorageAt
// TODO: Implement a variant that recieves the expected storage slot type and parses the result based on that
pub async fn get_storage_at(
    context: &CommandExecutionContext,
    account_id: NameOrAddress,
    slot: H256,
    block_id: Option<BlockId>,
) -> anyhow::Result<H256> {
    let storage_data = context
        .node_provider()
        .get_storage_at(account_id, slot, block_id)
        .await?;

    Ok(storage_data)
}
