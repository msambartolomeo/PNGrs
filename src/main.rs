use args::{Args, Commands::*};
use clap::Parser;
use commands::{decode, encode, print, remove};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Encode {
            path,
            code,
            message,
            output,
        } => encode(path, code, message, output),
        Decode { path, code } => decode(path, code),
        Remove { path, code } => remove(path, code),
        Print { path } => print(path),
    }
}
