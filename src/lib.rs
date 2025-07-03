use std::fmt;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CompactSize {
    pub value: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BitcoinError {
    InsufficientBytes,
    InvalidFormat,
}

impl CompactSize {
    pub fn new(value: u64) -> Self { Self { value } }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = match self.value {
            0x00..=0xfc => Vec::with_capacity(1),
            0xfd..=0xffff => Vec::with_capacity(3),
            0x10000..=0xffffffff => Vec::with_capacity(5),
            _ => Vec::with_capacity(9),
        };

        match self.value {
            0..=252 => bytes.push(self.value as u8),
            253..=65535 => {
                bytes.push(0xfd);
                bytes.extend_from_slice(&(self.value as u16).to_le_bytes());
            },
            65536..=4294967295 => {
                bytes.push(0xfe);
                bytes.extend_from_slice(&(self.value as u32).to_le_bytes());
            },
            _ => {
                bytes.push(0xff);
                bytes.extend_from_slice(&self.value.to_le_bytes());
            },
        }

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        match bytes {
            [] => Err(BitcoinError::InsufficientBytes),
            [first @ 0x00..=0xfc, ..] => Ok((Self::new(*first as u64), 1)),
            [0xfd, rest @ ..] if rest.len() >= 2 => {
                let value = u16::from_le_bytes(rest[0..2].try_into().unwrap()) as u64;
                Ok((Self::new(value), 3))
            },
            [0xfe, rest @ ..] if rest.len() >= 4 => {
                let value = u32::from_le_bytes(rest[0..4].try_into().unwrap()) as u64;
                Ok((Self::new(value), 5))
            },
            [0xff, rest @ ..] if rest.len() >= 8 => {
                let value = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                Ok((Self::new(value), 9))
            },
            _ => Err(BitcoinError::InsufficientBytes),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Txid(pub [u8; 32]);

impl Serialize for Txid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Txid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct OutPoint {
    pub txid: Txid,
    pub vout: u32,
}

impl OutPoint {
    pub fn new(txid: [u8; 32], vout: u32) -> Self { todo!() }

    pub fn to_bytes(&self) -> Vec<u8> { todo!() }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> { todo!() }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Script {
    pub bytes: Vec<u8>,
}

impl Script {
    pub fn new(bytes: Vec<u8>) -> Self { todo!() }

    pub fn to_bytes(&self) -> Vec<u8> { todo!() }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> { todo!() }
}

impl Deref for Script {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target { todo!() }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub previous_output: OutPoint,
    pub script_sig:      Script,
    pub sequence:        u32,
}

impl TransactionInput {
    pub fn new(previous_output: OutPoint, script_sig: Script, sequence: u32) -> Self { todo!() }

    pub fn to_bytes(&self) -> Vec<u8> { todo!() }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> { todo!() }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BitcoinTransaction {
    pub version:   u32,
    pub inputs:    Vec<TransactionInput>,
    pub lock_time: u32,
}

impl BitcoinTransaction {
    pub fn new(version: u32, inputs: Vec<TransactionInput>, lock_time: u32) -> Self { todo!() }

    pub fn to_bytes(&self) -> Vec<u8> { todo!() }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> { todo!() }
}

impl fmt::Display for BitcoinTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { todo!() }
}
