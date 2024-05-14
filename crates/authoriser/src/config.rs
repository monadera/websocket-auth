use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub jwks_url: String,
    pub audience: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| serde_env::from_env::<Config>().expect("config to load fine"))
}
