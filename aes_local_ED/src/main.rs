use std::io::{self, Write};
mod api;
mod hex;

fn main() {
    println!("Welcome to the Encoder/Decoder");
    println!("1. Encode");
    println!("2. Decode");
    println!("3. Text Len Counter");
    print!("Please choose (1/2/3): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => handle_encode(),
        "2" => handle_decode(),
        "3" => count_len(),
        _ => println!("Invalid choice!")
    }
}


fn count_len(){
    let data = 
    loop {
        print!("Enter data: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if !input.is_empty() {
            break input;
        }
    };
    println!("Len : {}", data.len())
}

fn handle_encode() {
    
    let password = loop {
        print!("Enter password (must be exactly 32 characters): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if input.len() == 32 {
            break input;
        } else {
            println!("Password must be exactly 32 characters!");
        }
    };

    let password_bytes: [u8; 32] = password.as_bytes().try_into().unwrap();

    // Get and validate data
    let data = 
    loop {
        print!("Enter data (length must be divisible by 16): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if input.len() % 16 == 0 && !input.is_empty() {
            break input;
        } else {
            println!("Data length must be divisible by 16!");
        }
    };

    let data_bytes: Vec<[u8; 16]> = data.as_bytes()
        .chunks(16)
        .map(|chunk| {
            let mut array = [0u8; 16];
            array.copy_from_slice(chunk);
            array
        })
        .collect();

    match api::encode(&password_bytes, &data_bytes) {
        Ok(encoded) => println!("Encoded data: {}", hex::bytes_to_hex(&encoded)),
        Err(e) => println!("Error encoding data: {}", e)
    }
}

fn handle_decode() {
    // Get and validate password
    let password = loop {
        print!("Enter password (must be exactly 32 characters): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if input.len() == 32 {
            break input;
        } else {
            println!("Password must be exactly 32 characters!");
        }
    };

    // Convert password to [u8; 32]
    let password_bytes: [u8; 32] = password.as_bytes().try_into().unwrap();

    // Get and validate hex data
    let hex_data = loop {
        print!("Enter hex data (must be lowercase and length divisible by 16): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();
        println!("{}",input);
        println!("{}",input.len());
        if input.len() % 16 == 0 && !input.is_empty() {
            break input;
        } else {
            println!("Data must be lowercase hex and length must be divisible by 16!");
        }
    };

    // Convert hex string to Vec<[u8; 16]>
    let data_bytes = match hex::hex_to_bytes(&hex_data) {
        Ok(bytes) => bytes
            .chunks(16)
            .map(|chunk| {
                let mut array = [0u8; 16];
                array.copy_from_slice(chunk);
                array
            })
            .collect(),
        Err(e) => {
            println!("Invalid hex data: {}", e);
            return;
        }
    };

    match api::decode(&password_bytes, &data_bytes) {
        Ok(decoded) => {
            let decoded_str = String::from_utf8_lossy(&decoded);
            println!("Decoded data: {}", decoded_str);
        },
        Err(e) => println!("Error decoding data: {}", e)
    }
}


