use std::fmt::{Display, Formatter};

use crate::config::Config;
use crate::error::Error;
use crate::error::Error::ConfigError;

const ERR_CONFIG_NO_USER: &str = "No user specified in db config";
const ERR_CONFIG_NO_HOST: &str = "No host specified in db config";

impl Config {
    pub(crate) fn db(&self) -> Result<ConnectionString, Error> {
        let json = self["db"].clone();

        let maybe_user: Result<String, Error> = match json.get("user") {
            Some(x) => Ok(x.as_str().unwrap().to_string()),
            None => Err(ConfigError(ERR_CONFIG_NO_USER.to_string()))?,
        };

        let maybe_host: Result<String, Error> = match json.get("host") {
            Some(x) => Ok(x.as_str().unwrap().to_string()),
            None => Err(ConfigError(ERR_CONFIG_NO_HOST.to_string()))?,
        };

        Ok(ConnectionString {
            user: maybe_user.unwrap(),
            host: maybe_host.unwrap(),
            password: json
                .get("password")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string()),
            dbname: json
                .get("dbname")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string()),
            options: json
                .get("options")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string()),
            application_name: json
                .get("application_name")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string()),
            sslmode: json
                .get("sslmode")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string()),
            port: json
                .get("port")
                .and_then(|x| x.to_string().parse::<u16>().ok().map(|x| x.to_string())),
            connect_timeout: json
                .get("connect_timeout")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string()),
            keepalives: json
                .get("keepalives")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string()),
            keepalives_idle: json
                .get("keepalives_idle")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string()),
            target_session_attrs: json
                .get("target_session_attrs")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string()),
            channel_binding: json
                .get("channel_binding")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string()),
        })
    }
}

pub(crate) struct ConnectionString {
    user: String,
    host: String,

    password: Option<String>,
    dbname: Option<String>,
    options: Option<String>,
    application_name: Option<String>,
    sslmode: Option<String>,
    port: Option<String>,
    connect_timeout: Option<String>,
    keepalives: Option<String>,
    keepalives_idle: Option<String>,
    target_session_attrs: Option<String>,
    channel_binding: Option<String>,
}

impl ConnectionString {
    pub(crate) fn as_string(&self) -> String {
        let mut strings = Vec::new();

        strings.push(format!("user={}", self.user));
        strings.push(format!("host={}", self.host));

        let optionals = vec![
            ("password", &self.password),
            ("dbname", &self.dbname),
            ("options", &self.options),
            ("application_name", &self.application_name),
            ("sslmode", &self.sslmode),
            ("port", &self.port),
            ("connect_timeout", &self.connect_timeout),
            ("keepalives", &self.keepalives),
            ("keepalives_idle", &self.keepalives_idle),
            ("target_session_attrs", &self.target_session_attrs),
            ("channel_binding", &self.channel_binding),
        ];

        for (key, value) in optionals {
            if let Some(value) = value {
                strings.push(format!("{}={}", key, value));
            }
        }

        strings.join(" ")
    }
}

impl Default for ConnectionString {
    fn default() -> Self {
        ConnectionString {
            user: "postgres".to_string(),
            host: "localhost".to_string(),
            password: None,
            dbname: None,
            options: None,
            application_name: None,
            sslmode: None,
            port: None,
            connect_timeout: None,
            keepalives: None,
            keepalives_idle: None,
            target_session_attrs: None,
            channel_binding: None,
        }
    }
}

impl Display for ConnectionString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
