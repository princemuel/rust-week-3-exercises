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
    pub fn new(value: u64) -> Self { todo!() }

    pub fn to_bytes(&self) -> Vec<u8> { todo!() }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> { todo!() }
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
