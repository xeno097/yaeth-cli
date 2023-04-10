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
