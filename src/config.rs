use std::env;

const DEFAULT_KEY: &str = "changeme";
const DEFAULT_PATH: &str = "sharex.db";

struct Config {
    private_key: String,
    db_path: String,
}

impl Config {
    fn load() -> Self {
        let private_key = env::var("SX_PRIVATE_KEY").unwrap_or(DEFAULT_KEY.to_string());
        let db_path = env::var("SX_DB_PATH").unwrap_or(DEFAULT_PATH.to_string());

        Self {
            private_key: private_key,
            db_path: db_path,
        }
    }
}
