use std::env;
use dotenv::dotenv;

pub fn get_env(key: &str, default: &str) -> String {
    dotenv().ok();
    match env::var(key) {
        Ok(val) => val,
        Err(_) => default.to_string(),
    }
}