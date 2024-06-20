use crate::config::Config;
use crate::openai_rs::Client;

/// Get openai client instance
pub fn get_client() -> Client {
    let config = Config::load();

    println!("{:?}", config);
    if config.openai_api_key.is_empty() {
        panic!("Please set openai_key first")
    }
    Client::new(&config.openai_api_key)
}