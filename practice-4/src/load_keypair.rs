use crate::parse_key;
use solana_sdk::signature::{Keypair, Signer};
use std::env;

pub fn load_keypair() {
    //loading env variables
    let secret_key = env::var("SECRET_KEY").expect("PUBLIC_KEY variable is not setted up!");

    if secret_key.len() == 0 {
        panic!("No SECRET_KEY")
    }

    let parsed_key = parse_key::parse_string_key(&secret_key);

    let secret_key_bytes: Vec<u8> = parse_key::parse_u8_key(&parsed_key);

    if let Ok(wallet) = Keypair::from_bytes(&secret_key_bytes) {
        let public_key = Signer::pubkey(&wallet);
        println!("Loaded keypair: {}", public_key);
    }
}
