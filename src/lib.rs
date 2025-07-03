use std::fmt;
use std::ops::Deref;

use serde::de::Error as DeError;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct CompactSize {
    pub value: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BitcoinError {
    InsufficientBytes,
    InvalidFormat,
}

impl CompactSize {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = match self.value {
            ..=0xfc => Vec::with_capacity(1),
            0xfd..=0xffff => Vec::with_capacity(3),
            0x10000..=0xffffffff => Vec::with_capacity(5),
            _ => Vec::with_capacity(9),
        };

        match self.value {
            ..=0xfc => bytes.push(self.value as u8),
            0xfd..=0xffff => {
                bytes.push(0xfd);
                bytes.extend_from_slice(&(self.value as u16).to_le_bytes());
            },
            0x10000..=0xffffffff => {
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
            [uno @ ..=0xfc, ..] => Ok((Self::new(*uno as u64), 1)),
            [0xfd, rest @ ..] => {
                let slice = rest.get(..2).ok_or(BitcoinError::InsufficientBytes)?;
                let value = u16::from_le_bytes(slice.try_into().unwrap()) as u64;
                Ok((Self::new(value), 3))
            },
            [0xfe, rest @ ..] => {
                let slice = rest.get(..4).ok_or(BitcoinError::InsufficientBytes)?;
                let value = u32::from_le_bytes(slice.try_into().unwrap()) as u64;
                Ok((Self::new(value), 5))
            },
            [0xff, rest @ ..] => {
                let slice = rest.get(..8).ok_or(BitcoinError::InsufficientBytes)?;
                let value = u64::from_le_bytes(slice.try_into().unwrap());
                Ok((Self::new(value), 9))
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Txid(pub [u8; 32]);

impl Serialize for Txid {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // let mut bytes = self.0;
        // bytes.reverse();
        s.serialize_str(&hex::encode(self.0))
    }
}

impl<'de> Deserialize<'de> for Txid {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let hex_string = String::deserialize(d)?;

        let decoded =
            hex::decode(&hex_string).map_err(|_| DeError::custom("Invalid hex string"))?;

        let bytes = decoded
            .try_into()
            .map_err(|_| DeError::custom("Txid must be exactly 32 bytes"))?;

        Ok(Txid(bytes))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct OutPoint {
    pub txid: Txid,
    pub vout: u32,
}

impl OutPoint {
    pub fn new(txid: [u8; 32], vout: u32) -> Self {
        Self {
            vout,
            txid: Txid(txid),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(36);
        bytes.extend_from_slice(&self.txid.0);
        bytes.extend_from_slice(&self.vout.to_le_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        if bytes.len() < 36 {
            return Err(BitcoinError::InsufficientBytes);
        }
        let txid = (&bytes[..32]).try_into().unwrap();
        let vout = u32::from_le_bytes((&bytes[32..36]).try_into().unwrap());
        Ok((Self::new(txid, vout), 36))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Script {
    pub bytes: Vec<u8>,
}

impl Script {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let len = CompactSize::new(self.bytes.len() as u64);
        let len_bytes = len.to_bytes();

        let mut result = Vec::with_capacity(len_bytes.len() + self.bytes.len());
        result.extend_from_slice(&len_bytes);
        result.extend_from_slice(&self.bytes);
        result
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        let (len, len_bytes) = CompactSize::from_bytes(bytes)?;
        let len_script = len.value as usize;

        let total_needed = len_bytes + len_script;
        if bytes.len() < total_needed {
            return Err(BitcoinError::InsufficientBytes);
        }

        let script_bytes = bytes[len_bytes..total_needed].to_vec();
        Ok((Script::new(script_bytes), total_needed))
    }
}

impl Deref for Script {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub previous_output: OutPoint,
    pub script_sig: Script,
    pub sequence: u32,
}

impl TransactionInput {
    pub fn new(previous_output: OutPoint, script_sig: Script, sequence: u32) -> Self {
        Self {
            previous_output,
            script_sig,
            sequence,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let outpoint_bytes = self.previous_output.to_bytes();
        let script_bytes = self.script_sig.to_bytes();
        let sequence_bytes: [u8; 4] = self.sequence.to_le_bytes();

        let mut bytes = Vec::with_capacity(
            outpoint_bytes.len() + script_bytes.len() + sequence_bytes.len(),
        );

        bytes.extend_from_slice(&outpoint_bytes);
        bytes.extend_from_slice(&script_bytes);
        bytes.extend_from_slice(&sequence_bytes);
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        let mut cursor = 0;

        let (previous_output, outpoint_len) = OutPoint::from_bytes(&bytes[cursor..])?;
        cursor += outpoint_len;

        let (script_sig, script_len) = Script::from_bytes(&bytes[cursor..])?;
        cursor += script_len;

        if bytes.len() < cursor + 4 {
            return Err(BitcoinError::InsufficientBytes);
        }

        let sequence = u32::from_le_bytes([
            bytes[cursor],
            bytes[cursor + 1],
            bytes[cursor + 2],
            bytes[cursor + 3],
        ]);

        cursor += 4;

        Ok((
            TransactionInput::new(previous_output, script_sig, sequence),
            cursor,
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BitcoinTransaction {
    pub version: u32,
    pub inputs: Vec<TransactionInput>,
    pub lock_time: u32,
}

impl BitcoinTransaction {
    pub fn new(version: u32, inputs: Vec<TransactionInput>, lock_time: u32) -> Self {
        Self {
            version,
            inputs,
            lock_time,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let input_count = CompactSize::new(self.inputs.len() as u64);
        let input_count_bytes = input_count.to_bytes();

        // initial capacity = version + count + lock_time
        let mut bytes = Vec::with_capacity(4 + input_count_bytes.len() + 4);

        bytes.extend_from_slice(&self.version.to_le_bytes());
        bytes.extend_from_slice(&input_count_bytes);

        for input in &self.inputs {
            bytes.extend_from_slice(&input.to_bytes());
        }

        bytes.extend_from_slice(&self.lock_time.to_le_bytes());

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        let mut cursor = 0;

        if bytes.len() < 4 {
            return Err(BitcoinError::InsufficientBytes);
        }

        let version = u32::from_le_bytes((&bytes[0..4]).try_into().unwrap());
        cursor += 4;

        let (input_count, count_len) = CompactSize::from_bytes(&bytes[cursor..])?;
        cursor += count_len;

        let mut inputs = Vec::with_capacity(input_count.value as usize);

        for _ in 0..input_count.value {
            let (input, input_len) = TransactionInput::from_bytes(&bytes[cursor..])?;
            inputs.push(input);
            cursor += input_len;
        }

        if bytes.len() < cursor + 4 {
            return Err(BitcoinError::InsufficientBytes);
        }

        let lock_time = u32::from_le_bytes([
            bytes[cursor],
            bytes[cursor + 1],
            bytes[cursor + 2],
            bytes[cursor + 3],
        ]);
        cursor += 4;

        Ok((BitcoinTransaction::new(version, inputs, lock_time), cursor))
    }
}

impl fmt::Display for BitcoinTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bitcoin Transaction:::\n Version: {}\n Lock Time: {}\n Inputs ({}):\n",
            self.version,
            self.lock_time,
            self.inputs.len()
        )?;

        for (i, input) in self.inputs.iter().enumerate() {
            write!(
                f,
                "  Input: {}:::\nPrevious Output Vout: {}\nScript Sig Length: {}\nScript Sig: ",
                i,
                input.previous_output.vout,
                input.script_sig.bytes.len()
            )?;

            for &byte in &input.script_sig.bytes {
                write!(f, "{byte:02x}")?;
            }

            write!(f, "\n  Sequence: 0x{:08x}\n", input.sequence)?;
        }

        Ok(())
    }
}
