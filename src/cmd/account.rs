use ethers::{
    providers::Middleware,
    types::{BlockId, BlockNumber, Bytes, NameOrAddress, H256, U256},
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
    get_transaction_count(
        context,
        account_id,
        Some(BlockId::Number(BlockNumber::Pending)),
    )
    .await
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

#[cfg(test)]
mod tests {

    mod get_balance {
        use ethers::utils::parse_ether;

        use crate::cmd::{account::get_balance, helpers::test::setup_test};

        #[test]
        fn should_get_the_account_balance() -> anyhow::Result<()> {
            // Arrange
            let (execution_context, anvil) = setup_test()?;

            let account = *anvil.addresses().get(0).unwrap();

            // Default account balance in Anvil
            let expected_balance = parse_ether(10_000)?;

            // Act
            let res =
                execution_context.execute(get_balance(&execution_context, account.into(), None));

            // Assert
            assert!(res.is_ok());

            let balance = res.unwrap();
            assert_eq!(balance, expected_balance);

            Ok(())
        }
    }

    mod get_code {
        use crate::cmd::{account::get_code, helpers::test::setup_test};

        #[test]
        fn should_get_the_account_code() -> anyhow::Result<()> {
            // Arrange
            let (execution_context, anvil) = setup_test()?;

            let account = *anvil.addresses().get(0).unwrap();

            // Act
            let res = execution_context.execute(get_code(&execution_context, account.into(), None));

            // Assert
            assert!(res.is_ok());

            let bytecode = res.unwrap();
            assert_eq!(bytecode.len(), 0);

            Ok(())
        }
    }

    mod get_transaction_count {
        use ethers::types::U256;

        use crate::cmd::{account::get_transaction_count, helpers::test::setup_test};

        #[test]
        fn should_get_the_account_transaction_count() -> anyhow::Result<()> {
            // Arrange
            let (execution_context, anvil) = setup_test()?;

            let account = *anvil.addresses().get(0).unwrap();

            // Act
            let res = execution_context.execute(get_transaction_count(
                &execution_context,
                account.into(),
                None,
            ));

            // Assert
            assert!(res.is_ok());

            let transaction_count = res.unwrap();
            assert_eq!(transaction_count, U256::default());

            Ok(())
        }

        // TODO: add tests for nonce
    }

    mod get_storage_at {
        use ethers::types::H256;

        use crate::cmd::{account::get_storage_at, helpers::test::setup_test};

        #[test]
        fn should_get_the_storage_data_in_the_selected_slot() -> anyhow::Result<()> {
            // Arrange
            let (execution_context, anvil) = setup_test()?;

            let account = *anvil.addresses().get(0).unwrap();

            // Act
            let res = execution_context.execute(get_storage_at(
                &execution_context,
                account.into(),
                H256::default(),
                None,
            ));

            // Assert
            assert!(res.is_ok());

            let storage_data = res.unwrap();
            assert_eq!(storage_data, H256::default());

            Ok(())
        }
    }
}
