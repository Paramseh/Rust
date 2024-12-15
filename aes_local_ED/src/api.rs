mod aes;

pub fn encode(password: &[u8; 32], data: &Vec<[u8; 16]>) -> Result<Vec<u8>, String> {
    let mut result = Vec::new();
    
    // Encode each block with AES and add to result
    for block in data {
        let encoded = aes::aes_256_encode(password, block);
        result.extend_from_slice(&encoded);
    }
    
    Ok(result)
}

pub fn decode(password: &[u8; 32], data: &Vec<[u8; 16]>) -> Result<Vec<u8>, String> {
    let mut result = Vec::new();
    
    // Decode each block with AES and add to result
    for block in data {
        let decoded = aes::aes_256_decode(password, block);
        result.extend_from_slice(&decoded);
    }
    
    Ok(result)
}
