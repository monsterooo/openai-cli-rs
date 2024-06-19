use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Models {
    Gpt4o,
    Gpt35Turbo,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optionally, the model name defaults to GPT-4o
    #[arg(short, long, value_enum, default_value_t = Models::Gpt4o)]
    model: Models,
}
