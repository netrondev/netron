#[cfg(feature = "ssr")]
use crate::AppError;
#[cfg(feature = "ssr")]
use dotenvy::dotenv;
use std::env;

#[allow(non_snake_case)]
pub struct Settings {
    pub surrealdb_host: String,
    pub surrealdb_db: String,
    pub surrealdb_ns: String,
    pub surrealdb_user: String,
    pub surrealdb_pass: String,
}

#[cfg(feature = "ssr")]
pub fn get_env(key: &str) -> Result<String, AppError> {
    env::var(key)
        .map_err(|_| AppError::EnvVarError(format!("Environment variable {} is not set", key)))
}

#[cfg(not(feature = "ssr"))]
pub fn get_env(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("Environment variable {} is not set", key))
}

pub fn get_settings() -> Settings {
    #[cfg(feature = "ssr")]
    dotenv().ok();

    Settings {
        surrealdb_host: get_env("SURREALDB_HOST").unwrap(),
        surrealdb_db: get_env("SURREALDB_DB").unwrap(),
        surrealdb_ns: get_env("SURREALDB_NS").unwrap(),
        surrealdb_user: get_env("SURREALDB_USER").unwrap(),
        surrealdb_pass: get_env("SURREALDB_PASS").unwrap(),
    }
}
