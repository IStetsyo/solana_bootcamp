use crate::parse_key;
use solana_sdk::signature::{Keypair, Signer};
use std::env;

pub fn generate_keypair() {
    //loading env variables
    let secret_key = env::var("SECRET_KEY").expect("PUBLIC_KEY variable is not setted up!");

    if secret_key.len() == 0 {
        panic!("No SECRET_KEY")
    }

    let parsed_key = parse_key::parse_string_key(&secret_key);

    let secret_key_bytes: Vec<u8> = parse_key::parse_u8_key(&parsed_key);

    if let Ok(wallet) = Keypair::from_bytes(&secret_key_bytes) {
        let public_key = Signer::pubkey(&wallet);
        println!("Created keypair: {}", public_key);
    }
}

pub fn generate_by_start(start_substring: &str) -> Keypair {
    let mut keypair = Keypair::new();
    let mut step_count: u32 = 1;
    let mut public_key = Signer::pubkey(&keypair).to_string();

    let limit_filled = step_count >= u32::max_value();

    while !public_key.to_lowercase().starts_with(start_substring) || !limit_filled {
        keypair = Keypair::new();
        step_count += 1;
        public_key = Signer::pubkey(&keypair).to_string();
        println!("Step: {}, key: {}", step_count, public_key);
    }
    if limit_filled {
        println!("Seems like you attempted to many times....")
    }
    println!("Created keypair: {} for {} steps", public_key, step_count);
    return keypair;
}
