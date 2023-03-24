use crate::config::CliConfig;
use std::future::Future;
use tokio::runtime;

pub struct CommandExecutionContext {
    config: CliConfig,
    runtime: runtime::Runtime,
}

impl CommandExecutionContext {
    pub fn new(config: CliConfig) -> Self {
        Self {
            config,
            runtime: runtime::Runtime::new().unwrap(),
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
}
