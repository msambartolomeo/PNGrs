pub mod chunk_type;

use anyhow::{bail, Error, Result};
use chunk_type::ChunkType;
use crc::{Crc, CRC_32_ISO_HDLC};
use std::fmt::Display;
use thiserror::Error as ThisError;

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

#[derive(Debug, ThisError)]
pub enum ChunkError {
    #[error("Error creating Chunk, the Crc provided {0} is not equal to the calculated value {1}")]
    InvalidCrc(u32, u32),
    #[error("Error creating Chunk, Could not find Data Length")]
    NoDataLengthProvided,
    #[error("Error creating Chunk, Could not find Chunk Type")]
    NoChunkTypeProvided,
    #[error(
        "Error creating Chunk, the Data Length provided {0} is not equal to the actual value {1}"
    )]
    NonMatchingDataLength(usize, usize),
    #[error("Error creating Chunk, Could not find Crc")]
    NoCrcProvided,
}

impl Chunk {
    pub const LENGTH_LENGTH: usize = 4;
    pub const TYPE_LENGTH: usize = 4;
    pub const CRC_LENGTH: usize = 4;
    pub const METEDATA_LENGTH: usize =
        Chunk::CRC_LENGTH + Chunk::LENGTH_LENGTH + Chunk::TYPE_LENGTH;

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data
            .len()
            .try_into()
            .expect("Invalid data size for chunk creation");

        let crc = Self::calculate_crc(&chunk_type, &data);
        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }

    fn calculate_crc(chunk_type: &ChunkType, data: &[u8]) -> u32 {
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);

        let crc_data: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();

        crc.checksum(&crc_data)
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }

    pub fn data_as_string(&self) -> Result<String> {
        Ok(std::str::from_utf8(&self.data)?.to_string())
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        // NOTE: instead of split_at BufReader.read_exact can be used
        // Example:
        // let mut reader = BufReader::new(bytes);
        // let mut buffer: [u8; 4] = [0, 0, 0, 0];
        // reader.read_exact(&mut buffer)?;
        // let data_length = u32::from_be_bytes(buffer);

        if value.len() < Chunk::LENGTH_LENGTH {
            bail!(ChunkError::NoDataLengthProvided);
        }
        let (length, value) = value.split_at(Chunk::LENGTH_LENGTH);
        let length = u32::from_be_bytes(length.try_into()?);

        if value.len() < Chunk::TYPE_LENGTH {
            bail!(ChunkError::NoChunkTypeProvided);
        }
        let (chunk_code, value) = value.split_at(Chunk::TYPE_LENGTH);
        let chunk_code: [u8; Chunk::TYPE_LENGTH] = chunk_code.try_into()?;
        let chunk_type = ChunkType::try_from(chunk_code)?;

        if value.len() < length as usize {
            bail!(ChunkError::NonMatchingDataLength(
                length as usize,
                value.len(),
            ));
        }

        let (data, value) = value.split_at(length as usize);
        let data = data.to_vec();

        if value.len() < Chunk::CRC_LENGTH {
            bail!(ChunkError::NoCrcProvided);
        }

        let (crc, _) = value.split_at(Chunk::CRC_LENGTH);
        let crc = u32::from_be_bytes(crc.try_into()?);

        let actual_crc = Self::calculate_crc(&chunk_type, &data);
        if crc != actual_crc {
            bail!(ChunkError::InvalidCrc(crc, actual_crc));
        }

        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
