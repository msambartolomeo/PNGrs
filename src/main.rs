use args::{Args, Commands};
use clap::Parser;
use commands::{decode, encode, print, remove};

mod args;
mod commands;

use anyhow::Result;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Encode {
            path,
            code,
            message,
            output,
        } => encode(path, &code, message, output),
        Commands::Decode { path, code } => decode(&path, &code),
        Commands::Remove { path, code } => remove(&path, &code),
        Commands::Print { path } => print(&path),
    }
}
