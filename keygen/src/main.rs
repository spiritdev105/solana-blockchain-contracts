use std::rc::Rc;

use anchor_client::solana_sdk::signature::{keypair_from_seed, read_keypair_file, Keypair};
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::{Client, Cluster};

use anchor_lang::system_program;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("generate key!");
    let key = Keypair::new();
    let key_string = key.to_base58_string();
    println!("key_String: {}", key_string);

    let rkey = Keypair::from_base58_string(&key_string);
    let rkey_str = rkey.to_base58_string();
    println!("key_String: {}", rkey_str);

    Ok(())
}
