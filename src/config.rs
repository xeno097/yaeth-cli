use config::Config;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CliConfig {
    priv_key: Option<String>,
    rpc_url: String,
}

impl CliConfig {
    pub fn priv_key(&self) -> Option<String> {
        self.priv_key.clone()
    }

    pub fn rpc_url(&self) -> &str {
        self.rpc_url.as_str()
    }
}

#[derive(Default)]
pub struct ConfigOverrides {
    priv_key: Option<String>,
    rpc_url: Option<String>,
    config_file: Option<String>,
}

impl ConfigOverrides {
    pub fn new(
        priv_key: Option<String>,
        rpc_url: Option<String>,
        config_file: Option<String>,
    ) -> Self {
        Self {
            config_file,
            priv_key,
            rpc_url,
        }
    }
}

const DEFAULT_RPC_URL: &str = "http://localhost:8545";

pub fn get_config(overrides: ConfigOverrides) -> Result<CliConfig, config::ConfigError> {
    let mut builder = Config::builder();

    builder = builder.set_default("rpc_url", DEFAULT_RPC_URL)?;

    if let Some(config_file) = overrides.config_file {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");

        builder = builder.add_source(config::File::from(base_path.join(config_file)));
    }

    if let Some(priv_key) = overrides.priv_key {
        builder = builder.set_override("priv_key", priv_key)?;
    }

    if let Some(rpc_url) = overrides.rpc_url {
        builder = builder.set_override("rpc_url", rpc_url)?;
    }

    let cli_config = builder.build()?;

    cli_config.try_deserialize::<CliConfig>()
}

#[cfg(test)]
mod tests {
    use super::{get_config, ConfigOverrides};
    use crate::config::DEFAULT_RPC_URL;
    use ethers::{core::rand::thread_rng, prelude::k256::ecdsa::SigningKey};

    const TEST_CONFIG_FILES_BASE_PATH: &str = "tests/config/";
    const FILE_CONFIG_PRIV_KEY: &str =
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    const FILE_CONFIG_RPC_URL: &str = "https://eth-mainnet.g.alchemy.com/v2/someapikey";

    #[test]
    fn should_use_the_default_config_values_if_no_other_values_are_provided() {
        // Arrange
        let overrides = ConfigOverrides::default();

        // Act
        let res = get_config(overrides);

        // Assert
        let res = res.unwrap();

        assert!(res.priv_key.is_none());
        assert_eq!(res.rpc_url, DEFAULT_RPC_URL);
    }

    #[test]
    fn should_read_the_config_values_from_file() {
        // Setup
        let config_file_names = vec!["config.json", "config.yaml"];

        for config_file_name in config_file_names {
            // Arrange
            let overrides = ConfigOverrides::new(
                None,
                None,
                Some(format!("{TEST_CONFIG_FILES_BASE_PATH}{config_file_name}")),
            );

            // Act
            let res = get_config(overrides);

            // Assert
            let res = res.unwrap();

            assert!(res.priv_key.is_some());
            assert_eq!(res.priv_key.unwrap(), FILE_CONFIG_PRIV_KEY);
            assert_eq!(res.rpc_url, FILE_CONFIG_RPC_URL);
        }
    }

    #[test]
    fn should_use_the_override_values() {
        // Arrange
        let expected_priv_key = hex::encode(SigningKey::random(&mut thread_rng()).to_bytes());
        let expected_rpc_url: &str = "https://eth-mainnet.g.alchemy.com/v2/someotherapikey";

        let overrides = ConfigOverrides::new(
            Some(expected_priv_key.clone()),
            Some(expected_rpc_url.into()),
            Some(format!("{TEST_CONFIG_FILES_BASE_PATH}config.json")),
        );

        // Act
        let res = get_config(overrides);

        // Assert
        let res = res.unwrap();

        assert!(res.priv_key.is_some());
        assert_eq!(res.priv_key.unwrap(), expected_priv_key);
        assert_eq!(res.rpc_url, expected_rpc_url);
    }

    #[test]
    fn should_not_find_config_file() {
        // Arrange
        let overrides = ConfigOverrides::new(None, None, Some("config.json".into()));

        // Act
        let res = get_config(overrides);

        // Assert
        assert!(res.is_err());
    }
}
