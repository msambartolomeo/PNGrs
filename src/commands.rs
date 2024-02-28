use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{bail, Result};

use pngrs::{Chunk, ChunkType, Png};

pub fn encode(path: PathBuf, code: &str, message: String, output: Option<PathBuf>) -> Result<()> {
    let mut png = Png::from_file(&path)?;

    let chunk_type = ChunkType::from_str(code)?;

    let chunk = Chunk::new(chunk_type, message.into_bytes());

    png.append_chunk(chunk);

    let out_path = output.unwrap_or(path);

    let out_bytes = png.as_bytes();

    fs::write(out_path, out_bytes)?;

    Ok(())
}

pub fn decode(path: &Path, code: &str) -> Result<()> {
    let png = Png::from_file(path)?;

    let Some(chunk) = png.chunk_by_type(code) else {
        bail!("Could not find message encoded with code {code}")
    };

    let message = chunk.data_as_string()?;

    println!("The encoded message with code {code} is {message}");

    Ok(())
}

pub fn remove(path: &Path, code: &str) -> Result<()> {
    let mut png = Png::from_file(path)?;

    let chunk = png.remove_chunk(code)?;

    let out_bytes = png.as_bytes();

    fs::write(path, out_bytes)?;

    let message = chunk.data_as_string()?;

    println!("Removed message encoded with code {code}, it was {message}",);

    Ok(())
}

pub fn print(path: &Path) -> Result<()> {
    let png = Png::from_file(path)?;

    println!("List of possible messages");

    println!("{png}");

    Ok(())
}
