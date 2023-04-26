use anyhow::Ok;
use ethers::{
    providers::{Http, Middleware, PendingTransaction},
    types::{BlockId, Bytes, Transaction, TransactionReceipt, TransactionRequest, H256},
};
use serde::Serialize;

use crate::context::NodeProvider;

pub enum GetTransaction {
    TransactionHash(H256),
    BlockIdAndIdx(BlockId, usize),
}

pub async fn get_transaction(
    node_provider: &NodeProvider,
    get_by: GetTransaction,
) -> anyhow::Result<Option<Transaction>> {
    match get_by {
        GetTransaction::TransactionHash(hash) => get_transaction_by_hash(node_provider, hash).await,
        GetTransaction::BlockIdAndIdx(block_id, idx) => {
            get_transaction_block_id_and_idx(node_provider, block_id, idx).await
        }
    }
}

// eth_getTransactionByHash
async fn get_transaction_by_hash(
    node_provider: &NodeProvider,
    hash: H256,
) -> anyhow::Result<Option<Transaction>> {
    let tx = node_provider.get_transaction(hash).await?;

    Ok(tx)
}

// eth_getTransactionByBlockHashAndIndex || eth_getTransactionByBlockNumberAndIndex
async fn get_transaction_block_id_and_idx(
    node_provider: &NodeProvider,
    block_id: BlockId,
    idx: usize,
) -> anyhow::Result<Option<Transaction>> {
    let block = node_provider.get_block_with_txs(block_id).await?;

    if let Some(block) = block {
        let tx = block.transactions.get(idx).cloned();

        return Ok(tx);
    }

    Ok(None)
}

// eth_getTransactionReceipt
pub async fn get_transaction_receipt(
    node_provider: &NodeProvider,
    hash: H256,
) -> anyhow::Result<Option<TransactionReceipt>> {
    let receipt = node_provider.get_transaction_receipt(hash).await?;

    Ok(receipt)
}

pub enum TransactionKind {
    RawTransaction(Bytes),
    TypedTransaction(TransactionRequest),
}

pub struct SendTransactionOptions {
    tx_data: TransactionKind,
    wait: bool,
}

impl SendTransactionOptions {
    pub fn new(data: TransactionKind, wait: Option<bool>) -> Self {
        Self {
            tx_data: data,
            wait: wait.unwrap_or(false),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum SendTxResult {
    PendingTransaction(H256),
    Receipt(Option<TransactionReceipt>),
}

pub async fn send_transaction(
    node_provider: &NodeProvider,
    tx_data: SendTransactionOptions,
) -> anyhow::Result<SendTxResult> {
    let SendTransactionOptions { tx_data, wait } = tx_data;

    let pending_tx = match tx_data {
        TransactionKind::RawTransaction(raw_tx) => {
            send_raw_transaction(node_provider, raw_tx).await?
        }
        TransactionKind::TypedTransaction(tx) => send_typed_transaction(node_provider, tx).await?,
    };

    let res = if wait {
        SendTxResult::Receipt(pending_tx.await?)
    } else {
        SendTxResult::PendingTransaction(pending_tx.tx_hash())
    };

    Ok(res)
}

// eth_sendRawTransaction
async fn send_raw_transaction(
    node_provider: &NodeProvider,
    encoded_tx: Bytes,
) -> anyhow::Result<PendingTransaction<Http>> {
    let receipt = node_provider.send_raw_transaction(encoded_tx).await?;

    Ok(receipt)
}

async fn send_typed_transaction(
    node_provider: &NodeProvider,
    tx: TransactionRequest,
) -> anyhow::Result<PendingTransaction<Http>> {
    let receipt = node_provider.send_transaction(tx, None).await?;

    Ok(receipt)
}

pub struct SimulateTransactionOptions(TransactionRequest, Option<BlockId>);

impl SimulateTransactionOptions {
    pub fn new(tx: TransactionRequest, block_id: Option<BlockId>) -> Self {
        Self(tx, block_id)
    }
}

pub async fn call(
    node_provider: &NodeProvider,
    options: SimulateTransactionOptions,
) -> anyhow::Result<Bytes> {
    let res = node_provider.call(&options.0.into(), options.1).await?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    mod get_transaction {

        use ethers::{
            types::{BlockId, BlockNumber},
            utils::parse_ether,
        };

        use crate::cmd::{
            helpers::test::{generate_random_h256, send_tx_helper, setup_test},
            transaction::{get_transaction, GetTransaction},
        };

        #[tokio::test]
        async fn should_not_find_a_transaction() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test().await?;

            let tx_hash = generate_random_h256();

            // Act
            let res =
                get_transaction(&node_provider, GetTransaction::TransactionHash(tx_hash)).await;

            // Assert
            assert!(res.is_ok());
            assert!(res.unwrap().is_none());

            Ok(())
        }

        #[tokio::test]
        async fn should_find_a_transaction_by_hash_or_block_id_and_index() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, anvil) = setup_test().await?;

            let sender = *anvil.addresses().get(0).unwrap();
            let receiver = *anvil.addresses().get(1).unwrap();

            let value = parse_ether(1)?;

            let tx_receipt = send_tx_helper(&node_provider, sender, receiver, value).await?;

            let tx_hash = tx_receipt.transaction_hash;
            let block_hash = tx_receipt.block_hash.unwrap();
            let block_number = tx_receipt.block_number.unwrap();

            let tx_index = 0;

            let test_cases = vec![
                GetTransaction::TransactionHash(tx_hash),
                GetTransaction::BlockIdAndIdx(BlockId::Hash(block_hash), tx_index),
                GetTransaction::BlockIdAndIdx(
                    BlockId::Number(BlockNumber::Number(block_number)),
                    tx_index,
                ),
            ];

            for test_case in test_cases {
                // Act
                let res = get_transaction(&node_provider, test_case).await;

                // Assert
                assert!(res.is_ok());

                let maybe_tx = res.unwrap();
                assert!(maybe_tx.is_some());

                let tx = maybe_tx.unwrap();
                assert_eq!(tx.hash, tx_hash);
                assert_eq!(tx.from, sender);
                assert_eq!(tx.to.unwrap(), receiver);
            }

            Ok(())
        }
    }

    mod get_transaction_receipt {

        use ethers::utils::parse_ether;

        use crate::cmd::{
            helpers::test::{generate_random_h256, send_tx_helper, setup_test},
            transaction::get_transaction_receipt,
        };

        #[tokio::test]
        async fn should_not_find_a_transaction_receipt() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test().await?;

            let tx_hash = generate_random_h256();

            // Act
            let res = get_transaction_receipt(&node_provider, tx_hash).await;

            // Assert
            assert!(res.is_ok());
            assert!(res.unwrap().is_none());

            Ok(())
        }

        #[tokio::test]
        async fn should_find_a_transaction_receipt() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, anvil) = setup_test().await?;

            let sender = *anvil.addresses().get(0).unwrap();
            let receiver = *anvil.addresses().get(1).unwrap();

            let value = parse_ether(1)?;

            let tx_hash = send_tx_helper(&node_provider, sender, receiver, value)
                .await?
                .transaction_hash;

            // Act
            let res = get_transaction_receipt(&node_provider, tx_hash).await;

            // Assert
            assert!(res.is_ok());

            let maybe_tx_receipt = res.unwrap();
            assert!(maybe_tx_receipt.is_some());

            let tx_receipt = maybe_tx_receipt.unwrap();
            assert_eq!(tx_receipt.transaction_hash, tx_hash);
            assert_eq!(tx_receipt.from, sender);
            assert_eq!(tx_receipt.to.unwrap(), receiver);

            Ok(())
        }
    }

    mod send_transaction {
        use ethers::{
            signers::{LocalWallet, Signer},
            types::{
                transaction::eip2718::TypedTransaction, Bytes, TransactionRequest, H160, U256,
            },
            utils::Anvil,
        };

        use crate::{
            cmd::{
                helpers::test::setup_test,
                transaction::{
                    send_transaction, SendTransactionOptions, SendTxResult, TransactionKind,
                },
            },
            config::{get_config, ConfigOverrides},
            context::CommandExecutionContext,
        };

        fn get_raw_transaction(
            signer: &LocalWallet,
            receiver: H160,
            chain_id: u64,
            value: Option<U256>,
        ) -> Bytes {
            let mut tx: TypedTransaction = TransactionRequest::new()
                .to(receiver)
                .gas(30000)
                .gas_price(14_000_000_000_u128)
                .chain_id(chain_id)
                .into();

            if let Some(value) = value {
                tx.set_value(value);
            }

            let sig = signer.sign_transaction_sync(&tx);

            tx.rlp_signed(&sig)
        }

        #[tokio::test]
        async fn should_send_the_raw_transaction() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, anvil) = setup_test().await?;

            let receiver = *anvil.addresses().get(1).unwrap();
            let signer: LocalWallet = anvil.keys().get(0).unwrap().clone().into();

            let raw_tx = get_raw_transaction(&signer, receiver, anvil.chain_id(), None);

            // Act
            let res = send_transaction(
                &node_provider,
                SendTransactionOptions::new(TransactionKind::RawTransaction(raw_tx), None),
            )
            .await;

            // Assert
            assert!(res.is_ok());

            Ok(())
        }

        #[tokio::test]
        async fn should_send_the_typed_transaction() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, anvil) = setup_test().await?;

            let sender = *anvil.addresses().get(0).unwrap();
            let receiver = *anvil.addresses().get(1).unwrap();

            let typed_tx = TransactionRequest::new().from(sender).to(receiver);

            // Act
            let res = send_transaction(
                &node_provider,
                SendTransactionOptions::new(TransactionKind::TypedTransaction(typed_tx), None),
            )
            .await;

            // Assert
            assert!(res.is_ok());

            Ok(())
        }

        #[tokio::test]
        async fn should_return_the_transaction_hash_if_wait_is_false() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, anvil) = setup_test().await?;

            let receiver = *anvil.addresses().get(1).unwrap();
            let signer: LocalWallet = anvil.keys().get(0).unwrap().clone().into();

            let raw_tx = get_raw_transaction(&signer, receiver, anvil.chain_id(), None);

            // Act
            let res = send_transaction(
                &node_provider,
                SendTransactionOptions::new(TransactionKind::RawTransaction(raw_tx), Some(false)),
            )
            .await?;

            // Assert
            assert!(matches!(res, SendTxResult::PendingTransaction(_)));

            Ok(())
        }

        #[tokio::test]
        async fn should_return_the_transaction_receipt_if_wait_is_true() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, anvil) = setup_test().await?;

            let receiver = *anvil.addresses().get(1).unwrap();
            let signer: LocalWallet = anvil.keys().get(0).unwrap().clone().into();

            let raw_tx = get_raw_transaction(&signer, receiver, anvil.chain_id(), None);

            // Act
            let res = send_transaction(
                &node_provider,
                SendTransactionOptions::new(TransactionKind::RawTransaction(raw_tx), Some(true)),
            )
            .await?;

            // Assert
            assert!(matches!(res, SendTxResult::Receipt(_)));

            Ok(())
        }

        #[test]
        fn should_send_the_transaction_from_the_private_key_address() -> anyhow::Result<()> {
            // Arrange
            let anvil = Anvil::new().spawn();

            let receiver = *anvil.addresses().get(1).unwrap();
            let priv_key = hex::encode(anvil.keys().get(0).unwrap().to_be_bytes());
            let signer: LocalWallet = priv_key.parse()?;

            let overrides = ConfigOverrides::new(Some(priv_key), Some(anvil.endpoint()), None);

            let config = get_config(overrides)?;

            let execution_context = CommandExecutionContext::new(config)?;

            let typed_tx = TransactionRequest::new().to(receiver);

            // Act
            let res = execution_context.execute(send_transaction(
                execution_context.node_provider(),
                SendTransactionOptions::new(
                    TransactionKind::TypedTransaction(typed_tx),
                    Some(true),
                ),
            ))?;

            // Assert
            match res {
                SendTxResult::PendingTransaction(_) => panic!("Should be a receipt!"),
                SendTxResult::Receipt(r) => assert_eq!(r.unwrap().from, signer.address()),
            }

            Ok(())
        }
    }

    mod call {
        use ethers::types::TransactionRequest;

        use crate::cmd::{
            helpers::test::setup_test,
            transaction::{call, SimulateTransactionOptions},
        };

        #[tokio::test]
        async fn should_simulate_the_transaction() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, anvil) = setup_test().await?;

            let sender = *anvil.addresses().get(0).unwrap();
            let receiver = *anvil.addresses().get(1).unwrap();

            let typed_tx = TransactionRequest::new().from(sender).to(receiver);

            // Act
            let res = call(
                &node_provider,
                SimulateTransactionOptions::new(typed_tx, None),
            )
            .await;

            // Assert
            assert!(res.is_ok());

            Ok(())
        }
    }
}
