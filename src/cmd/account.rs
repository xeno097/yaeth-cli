use ethers::{
    providers::Middleware,
    types::{BlockId, NameOrAddress},
};

use crate::context::CommandExecutionContext;

pub async fn get_balance(
    context: &CommandExecutionContext,
    account_id: NameOrAddress,
    block_id: Option<BlockId>,
) -> anyhow::Result<()> {
    let balance = context
        .node_provider()
        .get_balance(account_id, block_id)
        .await?;

    println!("{:#?}", balance);

    Ok(())
}
