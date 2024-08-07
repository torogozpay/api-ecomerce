use lazy_static::lazy_static;
use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OpenApi {
    pub swagger: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Api {
    pub api_server: String,
    pub api_user: String,
    pub api_pass: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jwt {
    pub jwt_secret: String,
    pub jwt_secs: usize
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub log: Log,
    pub api: Api,
    pub jwt: Jwt,
    pub env: ENV,
    pub openapi: OpenApi
}

const CONFIG_FILE_PATH: &str = "./shared/src/config/Default.toml";
const CONFIG_FILE_PREFIX: &str = "./shared/src/config/";

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Development".into());
        let mut s = Config::new();
        s.set("env", env.clone())?;

        s.merge(File::with_name(CONFIG_FILE_PATH))?;
        s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;
        
        s.try_into()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Testing,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Testing => write!(f, "Testing"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Testing" => ENV::Testing,
            "Production" => ENV::Production,
            _ => ENV::Development,
        }
    }
}

lazy_static! {
    pub static ref CONFIG : Settings = Settings::new().expect("Config can be loaded");
}