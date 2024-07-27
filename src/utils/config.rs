use envconfig::Envconfig;
use serde::Deserialize;
use std::fs;
use std::str::FromStr;

use crate::cli::commands::get_cli_args;

const DEFAULT_CONFIG_FILE: &str = "config.toml";

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Provider {
    None,
    Deribit,
}

impl FromStr for Provider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "deribit" => Ok(Provider::Deribit),
            _ => Err(format!("Invalid value for Provider: {}", s)),
        }
    }
}

#[derive(Deserialize, Debug, Default, Envconfig, PartialEq)]
pub struct Config {
    #[envconfig(nested = true)]
    pub exchange: ExchangeConfig,
    #[envconfig(nested = true)]
    pub provider: ProviderConfig,
}

#[derive(Deserialize, Debug, Default, Envconfig, PartialEq)]
pub struct ExchangeConfig {
    #[envconfig(from = "EXCHANGE_DEPTH_LIMIT")]
    pub depth_limit: Option<usize>,
    #[envconfig(from = "EXCHANGE_INSTRUMENT")]
    pub instrument: Option<String>,
}

#[derive(Deserialize, Debug, Default, Envconfig, PartialEq)]
pub struct ProviderConfig {
    #[envconfig(from = "PROVIDER_NAME")]
    pub name: Option<Provider>,
}

impl Config {
    /// Read the configuration from different sources and merge them.
    pub fn read_config() -> Self {
        // Order is important: environment variables > toml file > command line arguments
        let mut config = Self::from_toml_file(DEFAULT_CONFIG_FILE);

        let env_config = Self::read_env();
        config.merge(env_config);

        let cli_config = Self::from_cli_args();
        config.merge(cli_config);

        config
    }

    fn read_env() -> Self {
        Config::init_from_env().expect("Unable to read environment variables")
    }

    fn from_toml_file(file: &str) -> Self {
        let content = fs::read_to_string(file);
        match content {
            Ok(content) => toml::from_str(&content).expect("Unable to parse config file"),
            Err(_) => {
                eprintln!("Config file '{}' not found, using default values.", file);
                Self::default()
            }
        }
    }

    fn from_cli_args() -> Self {
        let matches = get_cli_args();

        // Extract command-line arguments
        let depth_limit = matches.get_one::<usize>("depth_limit").cloned();
        let instrument = matches.get_one::<String>("instrument").cloned();
        let provider_name = matches.get_one::<String>("provider").and_then(|s| s.parse().ok());

        Config {
            exchange: ExchangeConfig {
                depth_limit,
                instrument,
            },
            provider: ProviderConfig {
                name: provider_name,
            },
        }
    }

    /// Merge another Config into self, giving priority to non-None values of the other Config.
    fn merge(&mut self, other: Self) {
        if let Some(depth_limit) = other.exchange.depth_limit {
            self.exchange.depth_limit = Some(depth_limit);
        }
        if let Some(instrument) = other.exchange.instrument {
            self.exchange.instrument = Some(instrument);
        }
        if let Some(name) = other.provider.name {
            self.provider.name = Some(name);
        }
    }

    /// Validate that all necessary configuration fields are present and throw an error if any are invalid.
    pub fn validate(&self) {
        if self.exchange.instrument.is_none() {
            panic!("Instrument not specified in the configuration!");
        }
        if self.exchange.depth_limit.is_none() {
            panic!("Depth limit not specified in the configuration!");
        }
        if !matches!(self.exchange.depth_limit.unwrap(), 1 | 10 | 20) {
            panic!("Depth limit must be one of the following values: 1, 10, or 20!");
        }
        if self.provider.name.is_none() {
            panic!("Provider type not specified in the configuration!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn setup_test_env() {
        env::set_var("EXCHANGE_DEPTH_LIMIT", "10");
        env::set_var("EXCHANGE_INSTRUMENT", "BTC-USD");
        env::set_var("PROVIDER_NAME", "deribit");
    }

    fn teardown_test_env() {
        env::remove_var("EXCHANGE_DEPTH_LIMIT");
        env::remove_var("EXCHANGE_INSTRUMENT");
        env::remove_var("PROVIDER_NAME");
    }

    #[test]
    fn test_from_env() {
        setup_test_env();
        let config = Config::read_env();
        let expected = Config {
            exchange: ExchangeConfig {
                depth_limit: Some(10),
                instrument: Some("BTC-USD".to_string()),
            },
            provider: ProviderConfig {
                name: Some(Provider::Deribit),
            },
        };
        assert_eq!(config, expected);
        teardown_test_env();
    }

    #[test]
    fn test_merge_configs() {
        let mut file_config = Config {
            exchange: ExchangeConfig {
                depth_limit: Some(5),
                instrument: Some("BTC-ETH".to_string()),
            },
            provider: ProviderConfig {
                name: Some(Provider::Deribit),
            },
        };

        let env_config = Config {
            exchange: ExchangeConfig {
                depth_limit: Some(10),
                instrument: Some("BTC-USD".to_string()),
            },
            provider: ProviderConfig {
                name: None,
            },
        };

        file_config.merge(env_config);

        let expected = Config {
            exchange: ExchangeConfig {
                depth_limit: Some(10),
                instrument: Some("BTC-USD".to_string()),
            },
            provider: ProviderConfig {
                name: Some(Provider::Deribit),
            },
        };

        assert_eq!(file_config, expected);
    }

    #[test]
    #[should_panic(expected = "Instrument not specified in the configuration!")]
    fn test_validate_missing_instrument() {
        let config = Config {
            exchange: ExchangeConfig {
                depth_limit: Some(10),
                instrument: None,
            },
            provider: ProviderConfig {
                name: Some(Provider::Deribit),
            },
        };
        config.validate();
    }

    #[test]
    #[should_panic(expected = "Depth limit not specified in the configuration!")]
    fn test_validate_missing_depth_limit() {
        let config = Config {
            exchange: ExchangeConfig {
                depth_limit: None,
                instrument: Some("BTC-USD".to_string()),
            },
            provider: ProviderConfig {
                name: Some(Provider::Deribit),
            },
        };
        config.validate();
    }

    #[test]
    #[should_panic(expected = "Provider type not specified in the configuration!")]
    fn test_validate_missing_provider() {
        let config = Config {
            exchange: ExchangeConfig {
                depth_limit: Some(10),
                instrument: Some("BTC-USD".to_string()),
            },
            provider: ProviderConfig {
                name: None,
            },
        };
        config.validate();
    }

    #[test]
    #[should_panic(expected = "Depth limit must be one of the following values: 1, 10, or 20!")]
    fn test_validate_invalid_depth_limit() {
        let config = Config {
            exchange: ExchangeConfig {
                depth_limit: Some(5),
                instrument: Some("BTC-USD".to_string()),
            },
            provider: ProviderConfig {
                name: Some(Provider::Deribit),
            },
        };
        config.validate();
    }
}