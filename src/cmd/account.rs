use ethers::{
    providers::Middleware,
    types::{BlockId, NameOrAddress, U256},
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
