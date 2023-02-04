#![deny(unconditional_recursion)]
use std::{str::FromStr, error::Error, fmt, backtrace::Backtrace, num::ParseIntError};
use dotenv::dotenv;
use strum::ParseError;
use thiserror::Error;
use strum_macros::{EnumString, AsRefStr};

#[derive(Debug, Clone)]
pub struct Configs {
    pub auth_key: AuthKey,
    pub port: Port,
    pub address: Address,
    pub request_interval: RequestInterval,
    pub script_delay: ScriptDelay,
}

#[derive(Debug, Error)]
pub enum ConfigError{
    #[error("Unable to find env var: `{0}`")]
    MissingError(String),
    #[error("Error while parsing env vars: `{0}`")]
    ParseError(String),
    #[error("Error with strum: `{0}`")]
    StrumParseError(#[from] ParseError),
    #[error("Error with strum: `{0}`")]
    ParseIntError(#[from] ParseIntError),
}

#[derive(AsRefStr, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
enum ConfigValues {
    AuthKey(AuthKey),
    Port(Port),
    Address(Address),
    RequestInterval(RequestInterval),
    ScriptDelay(ScriptDelay),
}

pub type AuthKey = String;
pub type Port = u16;
pub type Address = String;
pub type RequestInterval = u8;
pub type ScriptDelay = u8;

pub trait ConfigValue {}

impl Configs {
    pub fn load() -> Result<Self, ConfigError> {
        Self::load_env_vars();

        let auth_key = match Self::get_var( "AUTH_KEY".to_string())? {
            Some(ConfigValues::AuthKey(c)) => c,
            Some(_) => return Err(ConfigError::ParseError("error parsing auth_key from .env file, please check it and try again".into())),
            None => return Err(ConfigError::MissingError("unable to find auth_key in .env".to_string()))
        };

        let port = match Self::get_var( "PORT".to_string())? {
            Some(ConfigValues::Port(c)) => c,
            Some(_) => return Err(ConfigError::ParseError("error parsing port from .env file, please check it and try again".to_string())),
            None => {
                println!("didn\'t find port in .env, using default of 8384");
                8384
            }
        };

        let address = match Self::get_var( "ADDRESS".to_string())? {
            Some(ConfigValues::Address(c)) => c,
            Some(_) => return Err(ConfigError::ParseError("error parsing address from .env file, please check it and try again".to_string())),
            None => {
                println!("didn\'t find address in .env, assuming localhost");
                "http://127.0.0.1".to_string()
            }
        };

        let request_interval = match Self::get_var( "REQUEST_INTERVAL".to_string())? {
            Some(ConfigValues::RequestInterval(c)) => c,
            Some(_) => return Err(ConfigError::ParseError("error parsing request interval from .env file, please check it and try again".to_string())),
            None => {
                println!("didn\'t find a request_interval in .env, using default of every 60 seconds");
                60
            }
        };

        let script_delay = match Self::get_var( "SCRIPT_DELAY".to_string())? {
            Some(ConfigValues::ScriptDelay(c)) => c,
            Some(_) => return Err(ConfigError::ParseError("error parsing request interval from .env file, please check it and try again".to_string())),
            None => {
                println!("didn\'t find a script_delay time in .env, using default of 1 minute");
                1
            }
        };

        Ok(Configs {
            address,
            auth_key,
            port,
            request_interval,
            script_delay
        })
    }

    fn load_env_vars() -> () {
        match dotenv() {
            Ok(_) => {
                println!(".env file found, using...")
            }
            Err(_) => {
                println!(".env file not found, looking elsewhere")
            }
        };
    }

    fn get_var(config_value: String) -> Result<Option<ConfigValues>, ConfigError> {
        println!("looking for: {}", &config_value);
        let valid_var = match std::env::var(&config_value) {
            Ok(url) => url,
            Err(_) => return Ok(None)
        };

        let config_value_variant = match ConfigValues::from_str(&config_value) {
            Ok(val) => val,
            Err(e) => return Err(e.into()),
        };

        match config_value_variant {
            ConfigValues::AuthKey(_) => Ok(Some(ConfigValues::AuthKey(valid_var))),
            ConfigValues::Address(_) => Ok(Some(ConfigValues::Address(valid_var))),
            ConfigValues::Port(_) => Ok(Some(ConfigValues::Port(valid_var.parse::<u16>()?))),
            ConfigValues::RequestInterval(_) => Ok(Some(ConfigValues::RequestInterval(valid_var.parse::<u8>()?))),
            ConfigValues::ScriptDelay(_) => Ok(Some(ConfigValues::ScriptDelay(valid_var.parse::<u8>()?))),
        }

    }
}