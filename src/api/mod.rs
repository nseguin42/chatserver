use actix_web::{get, middleware, web, App, HttpServer, Responder};

use crate::config::Config;
use crate::error::Error;
use crate::error::Error::ConfigError;

mod channel;
mod messages;

const ERR_CONFIG_NO_ADDRESS: &str = "No address specified in api config";

pub struct ApiConfig {
    pub address: String,
}

impl Config {
    pub(crate) fn api(&self) -> Result<ApiConfig, Error> {
        let json = self["api"].clone();

        let maybe_address: Result<String, Error> = match json.get("address") {
            Some(x) => Ok(x.as_str().unwrap().to_string()),
            None => Err(ConfigError(ERR_CONFIG_NO_ADDRESS.to_string()))?,
        };

        Ok(ApiConfig {
            address: maybe_address.unwrap(),
        })
    }
}

#[get("/")]
pub async fn index() -> impl Responder {
    "Main index"
}

pub async fn start(config: &Config) -> std::io::Result<()> {
    let config = config.api().unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(web::scope("/messages").service(messages::messages_index))
            .service(web::scope("/channel").service(channel::channel_index))
    })
    .bind(config.address)?
    .run()
    .await
}
