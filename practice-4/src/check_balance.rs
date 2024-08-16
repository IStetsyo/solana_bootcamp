use crate::parse_key;
use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;

use std::env;
use std::str::FromStr;

pub fn check_balance(request_airdrop: bool) {
    let url = "https://api.devnet.solana.com";
    let client = RpcClient::new(url);

    let public_key = env::var("PUBLIC_KEY").expect("PUBLIC_KEY variable is not setted up!");
    if public_key.len() == 0 {
        panic!("No PUBLIC_KEY")
    }
    let parsed_key = parse_key::parse_string_key(&public_key);

    let pubkey = Pubkey::from_str(&parsed_key).expect("Invalid parsed key2");
    match client.get_balance(&pubkey) {
        Ok(balance) => {
            let balance_in_lamports = balance as f64 / LAMPORTS_PER_SOL as f64;
            println!("Wallet balance: {} lamports", balance_in_lamports);
        }
        Err(err) => {
            eprintln!("Error fetching balance: {:?}", err);
        }
    }

    if request_airdrop {
        match client.request_airdrop(&pubkey, LAMPORTS_PER_SOL) {
            Ok(sig) => loop {
                if let Ok(confirmed) = client.confirm_transaction(&sig) {
                    if confirmed {
                        println!("Transaction: {} Status: {}", sig, confirmed);
                        break;
                    }
                }
            },
            Err(_) => println!("Error requesting airdrop"),
        };
    }
}
