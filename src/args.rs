use std::path::PathBuf;

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
        path: PathBuf,
        code: String,
        message: String,
        output: Option<PathBuf>,
    },

    #[command(about = "Decode a message stored in a PNG file")]
    Decode { path: PathBuf, code: String },

    #[command(about = "Remove a message from a PNG file")]
    Remove { path: PathBuf, code: String },

    #[command(about = "Print a list of PNG chunks that can be searched for messages")]
    Print { path: PathBuf },
}
