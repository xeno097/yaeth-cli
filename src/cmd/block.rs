use crate::context::NodeProvider;
use anyhow::Ok;
use ethers::{
    providers::Middleware,
    types::{Block, BlockId, BlockNumber, Transaction, TransactionReceipt, H256, U256, U64},
};

#[derive(Debug)]
pub enum BlockKind {
    RawBlock(Block<H256>),
    BlockWithTransaction(Block<Transaction>),
}

// eth_getBlockByHash || eth_getBlockByNumber
pub async fn get_block(
    node_provider: &NodeProvider,
    block_id: BlockId,
    include_tx: bool,
) -> Result<Option<BlockKind>, anyhow::Error> {
    let res = if include_tx {
        get_block_with_txs(node_provider, block_id)
            .await?
            .map(BlockKind::BlockWithTransaction)
    } else {
        get_raw_block(node_provider, block_id)
            .await?
            .map(BlockKind::RawBlock)
    };

    Ok(res)
}

async fn get_raw_block(
    node_provider: &NodeProvider,
    block_id: BlockId,
) -> Result<Option<Block<H256>>, anyhow::Error> {
    let block = node_provider.get_block(block_id).await?;

    if let Some(block) = block {
        return Ok(Some(block));
    }

    Ok(None)
}

async fn get_block_with_txs(
    node_provider: &NodeProvider,
    block_id: BlockId,
) -> Result<Option<Block<Transaction>>, anyhow::Error> {
    let block = node_provider.get_block_with_txs(block_id).await?;

    if let Some(block) = block {
        return Ok(Some(block));
    }

    Ok(None)
}

// eth_blockNumber
pub async fn get_block_number(node_provider: &NodeProvider) -> Result<U64, anyhow::Error> {
    let block_number = node_provider.get_block_number().await?;

    Ok(block_number)
}

// eth_getBlockTransactionCountByHash || eth_getBlockTransactionCountByNumber
pub async fn get_transaction_count(
    node_provider: &NodeProvider,
    block_id: BlockId,
) -> Result<Option<U256>, anyhow::Error> {
    if let Some(block) = get_raw_block(node_provider, block_id).await? {
        return Ok(Some(U256::from(block.transactions.len())));
    }

    Ok(None)
}

// eth_getUncleCountByBlockHash || eth_getUncleCountByBlockNumber
pub async fn get_uncle_block_count(
    node_provider: &NodeProvider,
    block_id: BlockId,
) -> Result<U256, anyhow::Error> {
    let count = node_provider.get_uncle_count(block_id).await?;

    Ok(count)
}

// eth_getBlockReceipts
pub async fn get_block_receipts(
    node_provider: &NodeProvider,
    block_id: BlockId,
) -> Result<Option<Vec<TransactionReceipt>>, anyhow::Error> {
    let block_id: BlockNumber = match block_id {
        BlockId::Hash(hash) => match get_raw_block(node_provider, hash.into()).await? {
            Some(block) => BlockNumber::from(block.number.ok_or(anyhow::anyhow!(
                "Block number not found for the block with the provided block hash"
            ))?),
            None => return Ok(None),
        },
        BlockId::Number(num) => num,
    };

    let receipts = node_provider.get_block_receipts(block_id).await?;

    Ok(Some(receipts))
}

#[cfg(test)]
mod tests {

    mod get_block {
        use ethers::types::{BlockId, BlockNumber};

        use crate::cmd::{
            block::{get_block, BlockKind},
            helpers::test::setup_test_with_no_context,
        };

        #[tokio::test]
        async fn should_not_find_a_non_existing_block() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test_with_no_context().await?;

            // Act
            let res = get_block(
                &node_provider,
                BlockId::Number(BlockNumber::Number(100.into())),
                false,
            )
            .await;

            // Assert
            assert!(res.is_ok());

            let maybe_block = res.unwrap();
            assert!(maybe_block.is_none());

            Ok(())
        }

        #[tokio::test]
        async fn should_get_the_block() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test_with_no_context().await?;

            // Act
            let res = get_block(&node_provider, BlockId::Number(BlockNumber::Latest), false).await;

            // Assert
            assert!(res.is_ok());

            let maybe_block = res.unwrap();
            assert!(maybe_block.is_some());

            Ok(())
        }

        #[tokio::test]
        async fn should_get_the_block_without_transactions() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test_with_no_context().await?;

            // Act
            let res = get_block(&node_provider, BlockId::Number(BlockNumber::Latest), false).await;

            // Assert
            assert!(res.is_ok());

            let maybe_block = res.unwrap();
            assert!(maybe_block.is_some());

            assert!(matches!(maybe_block.unwrap(), BlockKind::RawBlock(_)));

            Ok(())
        }

        #[tokio::test]
        async fn should_get_the_block_with_transactions() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test_with_no_context().await?;

            // Act
            let res = get_block(&node_provider, BlockId::Number(BlockNumber::Latest), true).await;

            // Assert
            assert!(res.is_ok());

            let maybe_block = res.unwrap();
            assert!(maybe_block.is_some());

            assert!(matches!(
                maybe_block.unwrap(),
                BlockKind::BlockWithTransaction(_)
            ));

            Ok(())
        }
    }

    mod get_block_number {
        use ethers::types::U64;

        use crate::cmd::{block::get_block_number, helpers::test::setup_test_with_no_context};

        #[tokio::test]
        async fn should_get_the_block_number() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test_with_no_context().await?;

            // Act
            let res = get_block_number(&node_provider).await;

            // Assert
            assert!(res.is_ok());

            let block_number = res.unwrap();
            assert_eq!(block_number, U64::default());

            Ok(())
        }
    }

    mod get_transaction_count {
        use ethers::types::{BlockId, BlockNumber, U256};

        use crate::cmd::{block::get_transaction_count, helpers::test::setup_test_with_no_context};

        #[tokio::test]
        async fn should_get_the_block_tx_count_for_an_existing_block() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test_with_no_context().await?;

            // Act
            let res =
                get_transaction_count(&node_provider, BlockId::Number(BlockNumber::Latest)).await;

            // Assert
            assert!(res.is_ok());

            let maybe_transaction_count = res.unwrap();
            assert!(maybe_transaction_count.is_some());

            let transaction_count = maybe_transaction_count.unwrap();
            assert_eq!(transaction_count, U256::default());

            Ok(())
        }

        #[tokio::test]
        async fn should_not_find_the_block_tx_count_for_a_non_existing_block() -> anyhow::Result<()>
        {
            // Arrange
            let (node_provider, _anvil) = setup_test_with_no_context().await?;

            // Act
            let res = get_transaction_count(
                &node_provider,
                BlockId::Number(BlockNumber::Number(100.into())),
            )
            .await;

            // Assert
            assert!(res.is_ok());

            let maybe_transaction_count = res.unwrap();
            assert!(maybe_transaction_count.is_none());

            Ok(())
        }
    }

    mod get_uncle_block_count {
        use ethers::types::{BlockId, BlockNumber, U256};

        use crate::cmd::{block::get_uncle_block_count, helpers::test::setup_test_with_no_context};

        #[tokio::test]
        async fn should_get_uncle_block_count() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test_with_no_context().await?;

            // Act
            let res =
                get_uncle_block_count(&node_provider, BlockId::Number(BlockNumber::Latest)).await;

            // Assert
            assert!(res.is_ok());

            let uncle_block_count = res.unwrap();
            assert_eq!(uncle_block_count, U256::default());

            Ok(())
        }
    }

    // Not testing  get_block_receipts because anvil does not support it
}
