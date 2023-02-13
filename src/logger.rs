use log::{debug, LevelFilter};
use pretty_env_logger::formatted_builder;
use serde_json::Value as JsonValue;

use crate::config::Config;
use crate::error::Error;
use crate::error::Error::Configuration;

const ERR_INVALID_LOG_LEVEL: &str = "Invalid log level";
const ERR_NO_LOG_LEVEL: &str = "No log level";

pub(crate) struct LoggerConfig {
    level: LevelFilter,
}

impl Config {
    pub(crate) fn logger(&self) -> Result<LoggerConfig, Error> {
        let json = self["logger"].clone();

        match Config::get_level(json) {
            Ok(level) => Ok(LoggerConfig { level }),
            Err(err) => panic!("{}", err),
        }
    }

    fn get_level(json: JsonValue) -> Result<LevelFilter, Error> {
        let val = json["level"]
            .as_str()
            .ok_or(Configuration(ERR_NO_LOG_LEVEL.to_string()))?;

        let maybe_match = match val {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            "off" => LevelFilter::Off,
            _ => Err(Configuration(ERR_INVALID_LOG_LEVEL.to_string()))?,
        };

        Ok(maybe_match)
    }
}

pub fn setup_logger(config: &Config) -> Result<(), Error> {
    let config = config.logger()?;
    let mut builder = formatted_builder();
    builder.filter(None, config.level).init();

    debug!("Logger initialized");
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_get_level() {
        let json = json!({
            "level": "trace"
        });
        let level = Config::get_level(json).unwrap();
        assert_eq!(level, LevelFilter::Trace);

        let json = json!({
            "level": "debug"
        });
        let level = Config::get_level(json).unwrap();
        assert_eq!(level, LevelFilter::Debug);

        let json = json!({
            "level": "info"
        });
        let level = Config::get_level(json).unwrap();
        assert_eq!(level, LevelFilter::Info);

        let json = json!({
            "level": "warn"
        });
        let level = Config::get_level(json).unwrap();
        assert_eq!(level, LevelFilter::Warn);

        let json = json!({
            "level": "error"
        });
        let level = Config::get_level(json).unwrap();
        assert_eq!(level, LevelFilter::Error);

        let json = json!({
            "level": "off"
        });
        let level = Config::get_level(json).unwrap();
        assert_eq!(level, LevelFilter::Off);

        let json = json!({
            "level": "invalid"
        });
        let level = Config::get_level(json);
        assert!(level.is_err());

        let json = json!({});
        let level = Config::get_level(json);
        assert!(level.is_err());
    }
}
