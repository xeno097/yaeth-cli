use ethers::{
    providers::Middleware,
    types::{BlockId, FeeHistory, TransactionRequest, U256},
};

use crate::context::NodeProvider;

use super::helpers::get_block_number_by_block_id;

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
    last_block_id: BlockId,
    reward_percentiles: Vec<f64>,
) -> anyhow::Result<Option<FeeHistory>> {
    if let Some(block_number) = get_block_number_by_block_id(node_provider, last_block_id).await? {
        let fee_history = node_provider
            .fee_history(block_count, block_number, &reward_percentiles)
            .await?;

        return Ok(Some(fee_history));
    }

    Ok(None)
}

// eth_gasPrice
pub async fn gas_price(node_provider: &NodeProvider) -> anyhow::Result<U256> {
    let current_gas_price = node_provider.get_gas_price().await?;

    Ok(current_gas_price)
}

// eth_maxPriorityFeePerGas
pub async fn get_max_priority_fee(node_provider: &NodeProvider) -> anyhow::Result<U256> {
    let current_max_priority_fee = node_provider.get_max_priority_fee_per_gas().await?;

    Ok(current_max_priority_fee)
}

#[cfg(test)]
mod tests {

    mod estimate_gas {
        use ethers::types::TransactionRequest;

        use crate::cmd::{gas::estimate_gas, helpers::test::setup_test};

        #[tokio::test]
        async fn should_get_the_gas_usage_estimation() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, anvil) = setup_test().await?;

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
        use ethers::types::{BlockNumber, H256};

        use crate::cmd::{gas::get_fee_history, helpers::test::setup_test};

        #[tokio::test]
        async fn should_get_the_fee_history() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test().await?;

            // Act
            let res = get_fee_history(
                &node_provider,
                10.into(),
                BlockNumber::Finalized.into(),
                [90.0, 97.7].into(),
            )
            .await;

            // Assert
            assert!(res.is_ok());
            let res = res.unwrap();

            assert!(res.is_some());

            Ok(())
        }

        #[tokio::test]
        async fn should_not_find_fee_history_for_non_existing_block() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test().await?;

            // Act
            let res = get_fee_history(
                &node_provider,
                10.into(),
                "0xef94b6d16908712a41ea538a21944826404d72be407bef0b050b7bab41300ec8"
                    .parse::<H256>()?
                    .into(),
                [90.0, 97.7].into(),
            )
            .await;

            // Assert
            assert!(res.is_ok());
            let res = res.unwrap();

            assert!(res.is_none());

            Ok(())
        }
    }

    mod gas_price {
        use crate::cmd::{gas::gas_price, helpers::test::setup_test};

        #[tokio::test]
        async fn should_get_the_gas_price() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test().await?;

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
        use crate::cmd::{gas::get_max_priority_fee, helpers::test::setup_test};

        #[tokio::test]
        async fn should_get_the_max_priority_fee() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test().await?;

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
