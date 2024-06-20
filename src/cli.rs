use clap::{Parser, Subcommand, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Models {
    Gpt4o,
    Gpt35Turbo,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optionally, the model name defaults to GPT-4o
    #[arg(short, long, value_enum, default_value_t = Models::Gpt4o)]
    pub model: Models,

    /// Questions to ask Openai
    #[arg(short, long)]
    pub question: String,

    #[command(subcommand)]
    pub config: Option<Config>
}

#[derive(Subcommand, Debug)]
pub enum Config {
    Save {
        /// Save the openai key to the configuration file
        #[arg(short, long)]
        openai_api_key: String
    }
}

impl AsRef<str> for Models {
    fn as_ref(&self) -> &str {
        match self {
            Models::Gpt35Turbo => "gpt-3.5-turbo",
            Models::Gpt4o => "gpt-4o"
        }
    }
}