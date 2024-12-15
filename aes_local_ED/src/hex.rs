pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        // Convert each byte to two hex digits
        hex.push(std::char::from_digit((byte >> 4) as u32, 16).unwrap());
        hex.push(std::char::from_digit((byte & 0xF) as u32, 16).unwrap());
    }
    hex
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have even length".to_string());
    }

    let mut bytes = Vec::with_capacity(hex.len() / 2);
    let mut chars = hex.chars();

    while let (Some(h1), Some(h2)) = (chars.next(), chars.next()) {
        let high = hex_char_to_value(h1)?;
        let low = hex_char_to_value(h2)?;
        bytes.push((high << 4) | low);
    }

    Ok(bytes)
}

fn hex_char_to_value(c: char) -> Result<u8, String> {
    match c {
        '0'..='9' => Ok(c as u8 - b'0'),
        'a'..='f' => Ok(c as u8 - b'a' + 10),
        _ => Err(format!("Invalid hex character: {}", c))
    }
}

