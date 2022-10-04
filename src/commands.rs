use crate::png::Png;
use crate::{Error, Result};

pub fn encode(path: String, code: String, message: String, output: Option<String>) -> Result<()> {
    let png = Png::from_file(&path)?;
}

pub fn decode(path: String, code: String) -> Result<()> {}

pub fn remove(path: String, code: String) -> Result<()> {}

pub fn print(path: String) -> Result<()> {}
