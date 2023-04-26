use ethers::{
    providers::Middleware,
    types::{Block, BlockId, BlockNumber, H256},
};

use crate::context::NodeProvider;

pub async fn get_raw_block(
    node_provider: &NodeProvider,
    block_id: BlockId,
) -> Result<Option<Block<H256>>, anyhow::Error> {
    let block = node_provider.get_block(block_id).await?;

    if let Some(block) = block {
        return Ok(Some(block));
    }

    Ok(None)
}

pub async fn get_block_number_by_block_id(
    node_provider: &NodeProvider,
    block_id: BlockId,
) -> anyhow::Result<Option<BlockNumber>> {
    let block_number = match block_id {
        BlockId::Hash(hash) => match get_raw_block(node_provider, hash.into()).await? {
            Some(block) => BlockNumber::from(block.number.ok_or(anyhow::anyhow!(
                "Block number not found for the block with the provided block hash"
            ))?),
            None => return Ok(None),
        },
        BlockId::Number(num) => num,
    };

    Ok(Some(block_number))
}

#[cfg(test)]
pub mod test {

    use ethers::{
        providers::Middleware,
        types::{TransactionReceipt, TransactionRequest, H160, H256, U256},
        utils::{Anvil, AnvilInstance},
    };
    use rand::Rng;

    use crate::{
        config::{get_config, ConfigOverrides},
        context::{CommandExecutionContext, NodeProvider},
    };

    pub fn setup_test() -> anyhow::Result<(CommandExecutionContext, AnvilInstance)> {
        let anvil = Anvil::new().spawn();

        let overrides = ConfigOverrides::new(None, Some(anvil.endpoint()), None);

        let config = get_config(overrides)?;

        let execution_context = CommandExecutionContext::new(config)?;

        Ok((execution_context, anvil))
    }

    pub async fn setup_test_with_no_context() -> anyhow::Result<(NodeProvider, AnvilInstance)> {
        let anvil = Anvil::new().spawn();

        let overrides = ConfigOverrides::new(None, Some(anvil.endpoint()), None);

        let config = get_config(overrides)?;

        let node_provider = NodeProvider::new(&config).await?;

        Ok((node_provider, anvil))
    }

    pub async fn send_tx_helper(
        node_provider: &NodeProvider,
        sender: H160,
        receiver: H160,
        value: U256,
    ) -> anyhow::Result<TransactionReceipt> {
        let tx = TransactionRequest::new()
            .value(value)
            .from(sender)
            .to(receiver);

        let tx = node_provider.send_transaction(tx, None).await?.await?;

        Ok(tx.unwrap())
    }

    pub fn generate_random_h256() -> H256 {
        let mut data = [0u8; 32];

        rand::thread_rng().fill(&mut data);

        data.into()
    }
}
