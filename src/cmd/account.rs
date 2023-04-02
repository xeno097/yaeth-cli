use ethers::{
    providers::Middleware,
    types::{BlockId, Bytes, NameOrAddress, U256},
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
