use std::error::Error;
use once_cell::sync::OnceCell;
use crate::shared::errors::error_enum::ErrorsEnum;

pub struct EnvLoader {
    url: String,
    port: u16,
    base_url: String,
    fixer_url: String,
    fixer_api_key: String,
    token_secret_key: String,
    database_url: String,
    connection_pool_max_size: u32,
    connection_pool_min_idle: Option<u32>,
}

impl EnvLoader {
    fn from_env() -> Result<Self, Box<dyn Error + Send + Sync>> {
        dotenvy::dotenv().ok();

        Ok(Self {
            url: std::env::var("URL")?,
            port: std::env::var("PORT")?.parse()?,
            base_url: std::env::var("BASE_URL")?,
            fixer_url: std::env::var("FIXER_URL")?,
            fixer_api_key: std::env::var("FIXER_API_KEY")?,
            token_secret_key: std::env::var("TOKEN_SECRET_KEY")?,
            database_url: std::env::var("DATABASE_URL")?,
            connection_pool_max_size: std::env::var("CONNECTION_POOL_MAX_SIZE")?.parse()?,
            connection_pool_min_idle: match std::env::var("CONNECTION_POOL_MIN_IDLE") {
                Ok(val) => Some(val.parse()?),
                Err(_) => None
            },
        })
    }

    pub fn get_url(&self) -> &str { &self.url }
    pub fn get_port(&self) -> u16 { self.port }
    pub fn get_base_url(&self) -> &str { &self.base_url }
    pub fn get_fixer_url(&self) -> &str { &self.fixer_url }
    pub fn get_fixer_api_key(&self) -> &str { &self.fixer_api_key }
    pub fn get_token_secret_key(&self) -> &str { &self.token_secret_key }
    pub fn get_database_url(&self) -> &str { &self.database_url }
    pub fn get_connection_pool_max_size(&self) -> u32 { self.connection_pool_max_size }
    pub fn get_connection_pool_min_idle(&self) -> Option<u32> { self.connection_pool_min_idle }

    /// Gets the address of the server including it's port
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.url, self.port)
    }

    /// Gets the fixer api-address with the api-key included
    pub fn get_fixer_address(&self) -> String {
        format!("{}{}", self.fixer_url, self.fixer_api_key)
    }
}

/// Global env loader
/// Is set once in the main function
/// If setting it fails the programm should terminate
static LOADER: OnceCell<EnvLoader> = OnceCell::new();
const ENV_LOADER_ERR_MSG: &str = "Environment could not be loaded";
pub fn get_loader() -> Result<&'static EnvLoader, ErrorsEnum> {
    match LOADER.get() {
        Some(loader) => Ok(loader),
        None => Err(ErrorsEnum::EnvLoaderError(ENV_LOADER_ERR_MSG.to_string()))
    }
}

pub fn set_loader() -> std::io::Result<()> {
    match EnvLoader::from_env() {
        Ok(env) => { match LOADER.set(env) {
            Ok(_) => Ok(()),
            Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to set env"))
        }},
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to load env"))
    }
}