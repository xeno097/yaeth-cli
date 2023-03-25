use crate::config::CliConfig;
use ethers::providers::{Http, Provider};
use std::future::Future;
use tokio::runtime;

pub struct CommandExecutionContext {
    config: CliConfig,
    runtime: runtime::Runtime,
    node_provider: Provider<Http>,
}

impl CommandExecutionContext {
    pub fn new(config: CliConfig) -> Self {
        let node_provider = ethers::providers::Provider::try_from(config.rpc_url()).unwrap();

        Self {
            config,
            runtime: runtime::Runtime::new().unwrap(),
            node_provider,
        }
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

    pub fn node_provider(&self) -> &Provider<Http> {
        &self.node_provider
    }
}
