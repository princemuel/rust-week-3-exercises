use rust_week_3_exercises::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_txid(val: u8) -> [u8; 32] {
        let mut txid = [0u8; 32];
        txid[31] = val;
        txid
    }

    #[test]
    fn test_compact_size_serialization() {
        let tests = vec![
            (0u64, vec![0x00]),
            (252u64, vec![0xfc]),
            (253u64, vec![0xfd, 0xfd, 0x00]),
            (65535u64, vec![0xfd, 0xff, 0xff]),
            (65536u64, vec![0xfe, 0x00, 0x00, 0x01, 0x00]),
            (4294967295u64, vec![0xfe, 0xff, 0xff, 0xff, 0xff]),
            (4294967296u64, vec![
                0xff, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            ]),
        ];

        for (value, bytes) in tests {
            let cs = CompactSize::new(value);
            assert_eq!(cs.to_bytes(), bytes);
            let (decoded, consumed) = CompactSize::from_bytes(&bytes).unwrap();
            debug_assert_eq!(decoded.value, value);
            debug_assert_eq!(consumed, bytes.len());
        }
    }

    #[test]
    fn test_outpoint_roundtrip() {
        let txid = dummy_txid(0xcc);
        let vout = 2;
        let outpoint = OutPoint::new(txid, vout);
        let bytes = outpoint.to_bytes();
        let (parsed, consumed) = OutPoint::from_bytes(&bytes).unwrap();
        assert_eq!(parsed, outpoint);
        assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_script_roundtrip() {
        let script_data = vec![0x76, 0xa9, 0x14, 0x88, 0xac];
        let script = Script::new(script_data.clone());
        let bytes = script.to_bytes();
        let (parsed, consumed) = Script::from_bytes(&bytes).unwrap();
        assert_eq!(parsed, script);
        assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_tx_input_roundtrip() {
        let outpoint = OutPoint::new(dummy_txid(1), 0);
        let script = Script::new(vec![0x01, 0x02]);
        let input = TransactionInput::new(outpoint.clone(), script.clone(), 0xffffffff);
        let bytes = input.to_bytes();
        let (parsed, consumed) = TransactionInput::from_bytes(&bytes).unwrap();
        assert_eq!(parsed, input);
        assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_bitcoin_tx_roundtrip() {
        let inputs = vec![TransactionInput::new(
            OutPoint::new(dummy_txid(1), 0),
            Script::new(vec![0x01, 0x02]),
            0xffffffff,
        )];
        let tx = BitcoinTransaction::new(2, inputs.clone(), 1000);
        let bytes = tx.to_bytes();
        let (parsed, consumed) = BitcoinTransaction::from_bytes(&bytes).unwrap();
        assert_eq!(parsed, tx);
        assert_eq!(consumed, bytes.len());
    }

    #[test]
    fn test_bitcoin_tx_json_serialization() {
        let input = TransactionInput::new(
            OutPoint::new(dummy_txid(0xab), 3),
            Script::new(vec![0xde, 0xad, 0xbe, 0xef]),
            0xabcdef01,
        );
        let tx = BitcoinTransaction::new(1, vec![input], 999);

        let json = serde_json::to_string_pretty(&tx).unwrap();
        let parsed: BitcoinTransaction = serde_json::from_str(&json).unwrap();
        assert_eq!(tx, parsed);

        assert!(json.contains("\"version\": 1"));
        assert!(json.contains("\"lock_time\": 999"));
    }

    #[test]
    fn test_bitcoin_transaction_display() {
        let input = TransactionInput::new(
            OutPoint::new(dummy_txid(0xcd), 7),
            Script::new(vec![0x01, 0x02, 0x03]),
            0xffffffff,
        );
        let tx = BitcoinTransaction::new(1, vec![input], 0);
        let output = format!("{tx}");
        assert!(output.contains("Version: 1"));
        assert!(output.contains("Lock Time: 0"));
        assert!(output.contains("Previous Output Vout: 7"));
    }
}
