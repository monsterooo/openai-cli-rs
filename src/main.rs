use anyhow::Result;
use clap::Parser;
use cli::{Cli, Config};
use config::Config as Conf;
use openai::get_client;
use openai_rs::chat::{ChatArguments, Message};

mod cli;
mod config;
mod openai;
mod openai_rs;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.config {
        Some(Config::Save { openai_api_key }) => {
            let mut config = Conf::load();
            config.openai_api_key = openai_api_key.clone();
            config.save().unwrap();
            println!("Saving the openai key succeeded.");
            return Ok(());
        }
        None => {}
    }

    let client = get_client();
    let args = ChatArguments::new(
        cli.model,
        vec![Message {
            role: "user".to_owned(),
            content: cli.question,
        }],
    );
    let res = client.create_chat(args).await.unwrap();
    println!("{}", res);

    Ok(())
}
