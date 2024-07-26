use std::rc::Rc;

use anchor_client::{
    solana_sdk::{
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        system_program,
    },
    Client, Cluster,
};
use counter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("program-id: {}", counter::ID);
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");
    let client = Client::new(Cluster::Localnet, Rc::new(payer));

    // Create program
    let program = client.program(counter::ID)?;

    // Send a transaction
    let account_kp = Keypair::new();

    let kp = Keypair::new();

    // Build and send a transaction.
    let tx = program
        .request()
        // .signer(&kp)
        .accounts(counter::Initialize {})
        .args(counter::instruction::Initialize {})
        .send()?;
    println!("tx: {:?}", tx);
    Ok(())
}
