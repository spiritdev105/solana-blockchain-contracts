use std::rc::Rc;

use anchor_client::solana_sdk::signature::{keypair_from_seed, read_keypair_file, Keypair};
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::{Client, Cluster};

use anchor_lang::system_program;
use counter::{accounts, instruction, Counter, ID};

const CLIENT_KEY_STR: &str =
    "4puAwQDQMsgv1466GctVMcQMAKxZ9qKN6tkB4dWWryqaoM5MbebA8wHC5qQn4VFTNKrSDNExX5LzzEsWr72zL3go";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Invoking counter contract!");
    println!("program-id: {}", counter::ID);
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("failed to get the keypair");
    let client = Client::new(Cluster::Devnet, Rc::new(payer));

    let program = client.program(counter::ID)?;

    // let counter = Keypair::new();
    let counter = Keypair::from_base58_string(&CLIENT_KEY_STR);
    let authority = program.payer();

    // initialize
    let tx = program
        .request()
        .signer(&counter)
        .accounts(accounts::InitializeCounter {
            counter: counter.pubkey(),
            payer: authority,
            system_program: system_program::ID,
        })
        .args(instruction::InitializeCounter {})
        .send()?;

    println!("tx: {:?}", tx);

    Ok(())
}
