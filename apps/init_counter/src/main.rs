use std::rc::Rc;

use anchor_client::solana_sdk::signature::{read_keypair_file, Keypair,keypair_from_seed};
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::{Client, Cluster};

use anchor_lang::system_program;
use counter::{accounts, instruction, Counter, ID};

const CLIENT_KEY_STR:&str = "3Y8Ahg2CkY7JpdFX7qoJDdwWjdv9oBCin5UQxTwQ5KSmqSNG3Zo9Mg2RCcVVJ46cQEJ1k676nZuq32erPzDFa7yM";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Invoking counter contract!");
    println!("program-id: {}", counter::ID);
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("failed to get the keypair");
    let client = Client::new(Cluster::Localnet, Rc::new(payer));

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


    // print current count value
    // let counter_account: Counter = program.account(counter.pubkey())?;
    // println!("count: {}", counter_account.count);

    // let tx = program
    //     .request()
    //     .accounts(accounts::Increment {
    //         counter: counter.pubkey(),
    //     })
    //     .args(instruction::Increment {})
    //     .send()?;

    // println!("tx: {:?}", tx);

    // let counter_account: Counter = program.account(counter.pubkey())?;
    // println!("count: {}", counter_account.count);

    Ok(())
}
