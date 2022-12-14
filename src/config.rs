use std::env;

const DEFAULT_KEY: &str = "changeme";
const DEFAULT_PATH: &str = "sharex.db";
const DEFAULT_SS_PATH: &str = "screenshots/";
const DEFAULT_DOMAIN: &str = "ss.ussr.pl";

// TODO: Learn macros to simplify this whole thing.
pub struct Config {
    pub private_key: String,
    pub db_path: String,
    pub screenshot_path: String,
    pub domain: String,
}

impl Config {
    pub fn load() -> Self {
        let private_key = env::var("SX_PRIVATE_KEY").unwrap_or(DEFAULT_KEY.to_string());
        let db_path = env::var("SX_DB_PATH").unwrap_or(DEFAULT_PATH.to_string());
        let screenshot_path = env::var("SX_SS_PATH").unwrap_or(DEFAULT_SS_PATH.to_string());
        let domain = env::var("SX_DOMAIN").unwrap_or(DEFAULT_DOMAIN.to_string());

        Self {
            private_key: private_key,
            db_path: db_path,
            screenshot_path: screenshot_path,
            domain: domain,
        }
    }
}
