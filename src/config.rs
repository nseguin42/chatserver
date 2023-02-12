use std::ops::Index;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    json: JsonValue,
}

impl Config {
    fn new(json: JsonValue) -> Self {
        Self { json }
    }

    fn from_key(json: JsonValue, key: &str) -> Result<Self, Error> {
        let json = json[key].clone();
        Ok(Self::new(json))
    }

    pub(crate) fn get(self, key: &str) -> Result<Self, Error> {
        Self::from_key(self.json, key)
    }

    pub(crate) async fn load(path: &str) -> Result<Self, Error> {
        let mut file = File::open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let json = serde_json::from_str(&*contents)?;
        Ok(Self { json })
    }
}

impl Index<&str> for Config {
    type Output = JsonValue;

    fn index(&self, key: &str) -> &Self::Output {
        &self.json[key]
    }
}
