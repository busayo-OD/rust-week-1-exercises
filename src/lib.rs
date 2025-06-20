// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    let raw_tx_bytes = hex::decode(raw_tx_hex).map_err(|e| format!("Hex decode error: {}", e))?;

    let version_bytes = &raw_tx_bytes[0..4];

    let mut bytes_array = [0u8; 4];
    bytes_array.copy_from_slice(&version_bytes);

    let version = u32::from_le_bytes(bytes_array);

    Ok(version)
}
