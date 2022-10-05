use anyhow::{bail, Result};
use pngrs::{Chunk, ChunkType, Png};
use std::fs;
use std::str::FromStr;

pub fn encode(path: String, code: String, message: String, output: Option<String>) -> Result<()> {
    let mut png = Png::from_file(&path)?;

    let chunk_type = ChunkType::from_str(&code)?;

    let chunk = Chunk::new(chunk_type, message.into_bytes());

    png.append_chunk(chunk);

    let out_path = output.unwrap_or(path);

    let out_bytes = png.as_bytes();

    fs::write(out_path, out_bytes)?;

    Ok(())
}

pub fn decode(path: String, code: String) -> Result<()> {
    let png = Png::from_file(&path)?;

    let chunk = match png.chunk_by_type(&code) {
        Some(chunk) => chunk,
        None => bail!("Could not find message encoded with code {}", code),
    };

    let message = chunk.data_as_string()?;

    println!("The encoded message with code {} is {}", code, message);

    Ok(())
}

pub fn remove(path: String, code: String) -> Result<()> {
    let mut png = Png::from_file(&path)?;

    let chunk = png.remove_chunk(&code)?;

    let out_bytes = png.as_bytes();

    fs::write(path, out_bytes)?;

    let message = chunk.data_as_string()?;

    println!(
        "Removed message encoded with code {}, it was {}",
        code, message
    );

    Ok(())
}

pub fn print(path: String) -> Result<()> {
    let png = Png::from_file(&path)?;

    println!("List of possible messages");

    println!("{}", png);

    Ok(())
}
