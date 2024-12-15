use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@?";

fn gen_pass() {
    // Generate a random 16-character password
    let mut rng = rand::thread_rng();
    let password: String = (0..16)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    // Print the password
    println!("Generated password: {}", password);
}

fn main() {
    gen_pass();
}
