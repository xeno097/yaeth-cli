use crate::config::CliConfig;
use async_trait::async_trait;
use ethers::{
    prelude::{
        k256::ecdsa::SigningKey, signer::SignerMiddlewareError, Middleware, SignerMiddleware,
    },
    providers::{Http, MiddlewareError, PendingTransaction, Provider, ProviderError},
    signers::{LocalWallet, Wallet},
    types::{transaction::eip2718::TypedTransaction, BlockId, U256},
};
use std::future::Future;
use thiserror::Error;
use tokio::runtime;

pub struct CommandExecutionContext {
    config: CliConfig,
    runtime: runtime::Runtime,
    node_provider: NodeProvider,
}

#[derive(Error, Debug)]
pub enum ExecutionContextError {
    #[error("{0}")]
    ProviderConfigError(NodeProviderConfigError),
}

impl CommandExecutionContext {
    pub fn new(config: CliConfig) -> Result<Self, ExecutionContextError> {
        let runtime = runtime::Runtime::new().unwrap();

        let node_provider = runtime
            .block_on(NodeProvider::new(&config))
            .map_err(ExecutionContextError::ProviderConfigError)?;

        Ok(Self {
            config,
            runtime,
            node_provider,
        })
    }

    pub fn execute<F>(&self, f: F) -> F::Output
    where
        F: Future,
    {
        self.runtime.block_on(f)
    }

    pub fn config(&self) -> &CliConfig {
        &self.config
    }

    pub fn node_provider(&self) -> &NodeProvider {
        &self.node_provider
    }
}

#[derive(Debug)]
pub enum NodeProvider {
    Provider(Provider<Http>),
    ProviderWithSigner(SignerMiddleware<Provider<Http>, Wallet<SigningKey>>),
}

impl NodeProvider {
    pub async fn new(config: &CliConfig) -> Result<Self, NodeProviderConfigError> {
        let provider = Provider::try_from(config.rpc_url())
            .map_err(|err| NodeProviderConfigError::InvalidProviderUrl(err.to_string()))?;

        let provider = if let Some(priv_key) = config.priv_key() {
            let signer = priv_key
                .parse::<LocalWallet>()
                .map_err(|err| NodeProviderConfigError::InvalidPrivateKey(err.to_string()))?;

            let signer_middleware = SignerMiddleware::new_with_provider_chain(provider, signer)
                .await
                .map_err(|err| NodeProviderConfigError::ProviderWithSignerError(err.to_string()))?;

            NodeProvider::ProviderWithSigner(signer_middleware)
        } else {
            NodeProvider::Provider(provider)
        };

        Ok(provider)
    }

    /// Returns the current max priority fee per gas in wei.
    pub async fn get_max_priority_fee_per_gas(&self) -> anyhow::Result<U256> {
        let res = self.inner().request("eth_maxPriorityFeePerGas", ()).await?;

        Ok(res)
    }
}

#[derive(Error, Debug)]
pub enum NodeProviderConfigError {
    #[error("{0}")]
    InvalidProviderUrl(String),

    #[error("{0}")]
    InvalidPrivateKey(String),

    #[error("{0}")]
    ProviderWithSignerError(String),
}

#[derive(Error, Debug)]
pub enum NodeProviderError {
    #[error("{0}")]
    ProviderError(ProviderError),

    #[error("{0}")]
    ProviderWithSignerError(SignerMiddlewareError<Provider<Http>, Wallet<SigningKey>>),
}

impl MiddlewareError for NodeProviderError {
    type Inner = ProviderError;

    fn from_err(src: ProviderError) -> Self {
        Self::ProviderError(src)
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        match self {
            NodeProviderError::ProviderError(err) => Some(err),
            _ => None,
        }
    }
}

// Config taken from the trait impl from https://github.com/gakonst/ethers-rs
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Middleware for NodeProvider {
    type Error = NodeProviderError;

    type Provider = Http;

    type Inner = Provider<Http>;

    fn inner(&self) -> &Self::Inner {
        match self {
            NodeProvider::Provider(provider) => provider,
            NodeProvider::ProviderWithSigner(provider_with_signer) => provider_with_signer.inner(),
        }
    }

    async fn send_transaction<T: Into<TypedTransaction> + Send + Sync>(
        &self,
        tx: T,
        block: Option<BlockId>,
    ) -> Result<PendingTransaction<'_, Http>, Self::Error> {
        match self {
            NodeProvider::Provider(provider) => provider
                .send_transaction(tx, block)
                .await
                .map_err(NodeProviderError::ProviderError),
            NodeProvider::ProviderWithSigner(signer_provider) => signer_provider
                .send_transaction(tx, block)
                .await
                .map_err(NodeProviderError::ProviderWithSignerError),
        }
    }
}
