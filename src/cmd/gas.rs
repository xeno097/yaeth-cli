use ethers::{
    providers::Middleware,
    types::{BlockId, BlockNumber, FeeHistory, TransactionRequest, U256},
};

use crate::context::NodeProvider;

// eth_estimateGas
pub async fn estimate_gas(
    node_provider: &NodeProvider,
    tx: TransactionRequest,
    block_id: Option<BlockId>,
) -> anyhow::Result<U256> {
    let estimated_gas = node_provider.estimate_gas(&tx.into(), block_id).await?;

    Ok(estimated_gas)
}

// eth_feeHistory
pub async fn get_fee_history(
    node_provider: &NodeProvider,
    block_count: U256,
    last_block: BlockNumber,
    reward_percentiles: Vec<f64>,
) -> anyhow::Result<FeeHistory> {
    let fee_history = node_provider
        .fee_history(block_count, last_block, &reward_percentiles)
        .await?;

    Ok(fee_history)
}

// eth_gasPrice
pub async fn gas_price(node_provider: &NodeProvider) -> anyhow::Result<U256> {
    let current_gas_price = node_provider.get_gas_price().await?;

    Ok(current_gas_price)
}

// eth_maxPriorityFeePerGas
pub async fn get_max_priority_fee(node_provider: &NodeProvider) -> anyhow::Result<U256> {
    let current_max_priority_fee = node_provider.get_eth_max_priority_fee_per_gas().await?;

    Ok(current_max_priority_fee)
}

#[cfg(test)]
mod tests {

    mod estimate_gas {
        use ethers::types::TransactionRequest;

        use crate::cmd::{gas::estimate_gas, helpers::test::setup_test_with_no_context};

        #[tokio::test]
        async fn should_get_the_gas_usage_estimation() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, anvil) = setup_test_with_no_context().await?;

            let sender = *anvil.addresses().get(0).unwrap();
            let receiver = *anvil.addresses().get(1).unwrap();

            let typed_tx = TransactionRequest::new().from(sender).to(receiver);

            let expected_gas = 21_000;

            // Act
            let res = estimate_gas(&node_provider, typed_tx, None).await;

            // Assert
            assert!(res.is_ok());
            let res = res.unwrap();

            assert_eq!(res, expected_gas.into());

            Ok(())
        }
    }

    mod get_fee_history {
        use ethers::types::BlockNumber;

        use crate::cmd::{gas::get_fee_history, helpers::test::setup_test_with_no_context};

        #[tokio::test]
        async fn should_get_the_gas_usage_estimation() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test_with_no_context().await?;

            // Act
            let res = get_fee_history(
                &node_provider,
                10.into(),
                BlockNumber::Finalized,
                [90.0, 97.7].into(),
            )
            .await;

            // Assert
            assert!(res.is_ok());

            Ok(())
        }
    }

    mod gas_price {
        use crate::cmd::{gas::gas_price, helpers::test::setup_test_with_no_context};

        #[tokio::test]
        async fn should_get_the_gas_price() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test_with_no_context().await?;

            // Act
            let res = gas_price(&node_provider).await;

            // Assert
            assert!(res.is_ok());
            let res = res.unwrap();

            assert!(res > 0.into());

            Ok(())
        }
    }

    mod get_max_priority_fee {
        use crate::cmd::{gas::get_max_priority_fee, helpers::test::setup_test_with_no_context};

        #[tokio::test]
        async fn should_get_max_priority_fee() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test_with_no_context().await?;

            // Act
            let res = get_max_priority_fee(&node_provider).await;

            // Assert
            assert!(res.is_ok());
            let res = res.unwrap();

            assert!(res > 0.into());

            Ok(())
        }
    }
}
