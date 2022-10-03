use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "PNGrs")]
#[command(author = "Mauro Sambartolomeo")]
#[command(version = "1.0")]
#[command(about = "Encode and decode messages into a PNG files")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Encode a message into a PNG file")]
    Encode {
        path: String,
        code: String,
        message: String,
        output: Option<String>,
    },

    #[command(about = "Decode a message stored in a PNG file")]
    Decode { path: String, code: String },

    #[command(about = "Remove a message from a PNG file")]
    Remove { path: String, code: String },

    #[command(about = "Print a list of PNG chunks that can be searched for messages")]
    Print { path: String },
}
