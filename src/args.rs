use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "PNGrs")]
#[command(author = "Mauro Sambartolomeo")]
#[command(version = "1.0")]
#[command(about = "Encodes a message inside a png image")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        path: String,
        code: String,
        message: String,
        output: Option<String>,
    },

    Decode {
        path: String,
        code: String,
    },

    Remove {
        path: String,
        code: String,
    },

    Print {
        path: String,
    },
}
