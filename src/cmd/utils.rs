use crate::context::NodeProvider;
use anyhow::Result;
use ethers::{
    providers::Middleware,
    types::{
        transaction::eip2718::TypedTransaction, Address, BlockId, Bytes, EIP1186ProofResponse,
        NameOrAddress, Signature, SyncingStatus, TransactionRequest, H160, H256, U256,
    },
};

// eth_accounts
pub async fn get_accounts(node_provider: &NodeProvider) -> Result<Vec<H160>> {
    let accounts = node_provider.get_accounts().await?;

    Ok(accounts)
}

// eth_chainId
pub async fn get_chain_id(node_provider: &NodeProvider) -> Result<U256> {
    let chain_id = node_provider.get_chainid().await?;

    Ok(chain_id)
}

// eth_getProof
pub async fn get_proof(
    node_provider: &NodeProvider,
    address: NameOrAddress,
    storage_locations: Vec<H256>,
    block_id: Option<BlockId>,
) -> Result<EIP1186ProofResponse> {
    let account_proof = node_provider
        .get_proof(address, storage_locations, block_id)
        .await?;

    Ok(account_proof)
}

pub async fn get_protocol_version(node_provider: &NodeProvider) -> Result<U256> {
    let protocol_version = node_provider.get_protocol_version().await?;

    Ok(protocol_version)
}

pub enum SignTransactionData {
    Raw(Bytes),
    Transaction(TransactionRequest),
}

pub async fn sign(
    node_provider: &NodeProvider,
    from: NameOrAddress,
    data: SignTransactionData,
) -> Result<Signature> {
    let from = match from {
        NameOrAddress::Name(ens) => node_provider.resolve_name(&ens).await?,
        NameOrAddress::Address(addr) => addr,
    };

    match data {
        SignTransactionData::Raw(data) => sign_raw_data(node_provider, from, data).await,
        SignTransactionData::Transaction(tx) => {
            sign_transaction(node_provider, from, tx.into()).await
        }
    }
}

async fn sign_raw_data(
    node_provider: &NodeProvider,
    from: Address,
    data: Bytes,
) -> Result<Signature> {
    let signature = node_provider.sign(data, &from).await?;

    Ok(signature)
}

async fn sign_transaction(
    node_provider: &NodeProvider,
    from: Address,
    tx: TypedTransaction,
) -> Result<Signature> {
    let signature = node_provider.sign_transaction(&tx, from).await?;

    Ok(signature)
}

pub async fn get_sync_status(node_provider: &NodeProvider) -> Result<SyncingStatus> {
    let sync_status = node_provider.syncing().await?;

    Ok(sync_status)
}

#[cfg(test)]
mod tests {

    mod get_accounts {

        use ethers::types::H160;

        use crate::cmd::{helpers::test::setup_test, utils::get_accounts};

        #[tokio::test]
        async fn should_get_the_accounts_known_by_the_node() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test().await?;

            let expected_res: [H160; 10] = [
                "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266".parse()?,
                "0x70997970c51812dc3a010c7d01b50e0d17dc79c8".parse()?,
                "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc".parse()?,
                "0x90f79bf6eb2c4f870365e785982e1f101e93b906".parse()?,
                "0x15d34aaf54267db7d7c367839aaf71a00a2c6a65".parse()?,
                "0x9965507d1a55bcc2695c58ba16fb37d819b0a4dc".parse()?,
                "0x976ea74026e726554db657fa54763abd0c3a0aa9".parse()?,
                "0x14dc79964da2c08b23698b3d3cc7ca32193d9955".parse()?,
                "0x23618e81e3f5cdf7f54c3d65f7fbc0abf5b21e8f".parse()?,
                "0xa0ee7a142d267c1f36714e4a8f75612f20a79720".parse()?,
            ];

            // Act
            let res = get_accounts(&node_provider).await;

            // Assert
            assert!(res.is_ok());

            let maybe_accounts = res.unwrap();
            assert_eq!(maybe_accounts.len(), 10);
            assert_eq!(maybe_accounts, expected_res);

            Ok(())
        }
    }

    mod get_chain_id {

        use ethers::types::U256;

        use crate::cmd::{helpers::test::setup_test, utils::get_chain_id};

        #[tokio::test]
        async fn should_get_the_chain_id() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test().await?;

            let expected_res: U256 = 31337.into();

            // Act
            let res = get_chain_id(&node_provider).await;

            // Assert
            assert!(res.is_ok());

            let maybe_chain_id = res.unwrap();
            assert_eq!(maybe_chain_id, expected_res);

            Ok(())
        }
    }

    mod get_proof {

        use ethers::utils::parse_ether;

        use crate::cmd::{helpers::test::setup_test, utils::get_proof};

        #[tokio::test]
        async fn should_get_the_account_merkle_proof() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, anvil) = setup_test().await?;

            let account = *anvil.addresses().get(0).unwrap();
            let expected_account_balance = parse_ether(10000)?;

            // Act
            let res = get_proof(&node_provider, account.into(), [].into(), None).await;

            // Assert
            assert!(res.is_ok());

            let maybe_account_proof = res.unwrap();
            assert_eq!(maybe_account_proof.address, account);
            assert_eq!(maybe_account_proof.balance, expected_account_balance);
            assert_eq!(maybe_account_proof.nonce, 0.into());

            Ok(())
        }
    }

    mod sign {
        use ethers::{
            types::{Bytes, RecoveryMessage, TransactionRequest, H160},
            utils::Anvil,
        };

        use crate::{
            cmd::{
                helpers::test::setup_test,
                utils::{sign, SignTransactionData},
            },
            config::{get_config, ConfigOverrides},
            context::NodeProvider,
        };

        #[tokio::test]
        async fn should_not_sign_the_data_if_with_an_unkwnon_signer() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test().await?;

            let bytes = SignTransactionData::Raw(Bytes::from_static(b"somerandomdata"));
            let from = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92267".parse::<H160>()?;

            // Act
            let res = sign(&node_provider, from.into(), bytes).await;

            // Assert
            assert!(res.is_err());

            let err = res.unwrap_err();
            assert!(err.to_string().contains("No Signer available"));

            Ok(())
        }

        #[tokio::test]
        async fn should_sign_the_data() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, anvil) = setup_test().await?;

            let bytes = Bytes::from_static(b"somerandomdata");
            let data = SignTransactionData::Raw(bytes.clone());
            let from = *anvil.addresses().get(0).unwrap();

            // Act
            let res = sign(&node_provider, from.into(), data).await;

            // Assert
            assert!(res.is_ok());

            let sig = res.unwrap();
            assert!(sig
                .verify(RecoveryMessage::Data(bytes.into_iter().collect()), from)
                .is_ok());

            Ok(())
        }

        #[tokio::test]
        async fn should_not_sign_the_tx_data_if_no_private_key_is_configured() -> anyhow::Result<()>
        {
            // Arrange
            let (node_provider, anvil) = setup_test().await?;

            let tx = TransactionRequest::new();
            let data = SignTransactionData::Transaction(tx);
            let from = *anvil.addresses().get(0).unwrap();

            // Act
            let res = sign(&node_provider, from.into(), data).await;

            // Assert
            assert!(res.is_err());

            Ok(())
        }

        #[tokio::test]
        async fn should_sign_the_tx_data() -> anyhow::Result<()> {
            // Arrange
            let anvil = Anvil::new().spawn();
            let priv_key = hex::encode(anvil.keys().get(0).unwrap().to_be_bytes());

            let overrides = ConfigOverrides::new(Some(priv_key), Some(anvil.endpoint()), None);
            let config = get_config(overrides)?;

            let node_provider = NodeProvider::new(&config).await?;

            let tx = TransactionRequest::new();
            let data = SignTransactionData::Transaction(tx.clone());
            let from = *anvil.addresses().get(0).unwrap();

            // Act
            let res = sign(&node_provider, from.into(), data).await;

            // Assert
            assert!(res.is_ok());

            Ok(())
        }
    }

    mod get_sync_status {

        use crate::cmd::{helpers::test::setup_test, utils::get_sync_status};

        #[tokio::test]
        async fn should_get_the_node_sync_status() -> anyhow::Result<()> {
            // Arrange
            let (node_provider, _anvil) = setup_test().await?;

            // Act
            let res = get_sync_status(&node_provider).await;

            // Assert
            assert!(res.is_ok());

            Ok(())
        }
    }
}
