use crate::config::Config;
use crate::error::Error;
use crate::error::Error::Configuration;

pub(crate) mod channel;
pub mod message;

const ERR_CONFIG_NO_ADDRESS: &str = "No address specified in api config";

pub struct ApiConfig {
    pub address: String,
}

impl Config {
    pub(crate) fn api(&self) -> Result<ApiConfig, Error> {
        let json = self["api"].clone();

        let maybe_address: Result<String, Error> = match json.get("address") {
            Some(x) => Ok(x.as_str().unwrap().to_string()),
            None => Err(Configuration(ERR_CONFIG_NO_ADDRESS.to_string()))?,
        };

        Ok(ApiConfig {
            address: maybe_address.unwrap(),
        })
    }
}
