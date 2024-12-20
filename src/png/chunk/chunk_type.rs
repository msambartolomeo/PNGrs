use anyhow::{bail, Error, Result};
use std::{fmt::Display, str::FromStr};
use thiserror::Error as ThisError;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    code: [u8; 4],
}

#[derive(Debug, ThisError)]
pub enum ChunkTypeError {
    #[error("Error creating chunk type, code is of length {0} but it must be of length 4")]
    InvalidLength(usize),
    #[error("Error creating chunk type, value {0} is not a valid ascii letter")]
    InvalidByte(u8),
}

// NOTE: Functions are allowed unused for future extension
#[allow(unused)]
impl ChunkType {
    #[must_use]
    pub const fn bytes(&self) -> [u8; 4] {
        self.code
    }

    const fn is_property_bit_on(&self, byte: usize) -> bool {
        assert!(
            byte != 0 && byte <= 4,
            "Index out of bounds, should not happen as it is a private method"
        );
        self.code[byte - 1] & (1 << 5) != 0
    }

    const fn is_critical(&self) -> bool {
        !self.is_property_bit_on(1)
    }

    const fn is_public(&self) -> bool {
        !self.is_property_bit_on(2)
    }

    const fn is_reserved_bit_valid(&self) -> bool {
        !self.is_property_bit_on(3)
    }

    const fn is_safe_to_copy(&self) -> bool {
        self.is_property_bit_on(4)
    }

    const fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        for c in value {
            if let 0..=64 | 91..=96 | 123..=255 = c {
                bail!(ChunkTypeError::InvalidByte(c));
            }
        }

        Ok(Self { code: value })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let code: [u8; 4] = match s.as_bytes().try_into() {
            Ok(code) => code,
            Err(_) => bail!(ChunkTypeError::InvalidLength(s.len())),
        };

        Self::try_from(code)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // NOTE: Should not fail because it is always chars
        unsafe { write!(f, "{}", std::str::from_utf8_unchecked(&self.code)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
