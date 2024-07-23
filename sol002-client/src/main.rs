use std::rc::Rc;

use anchor_client::{
    solana_sdk::{
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        system_program,
    },
    Client, Cluster,
};

use my_program::{accounts, instruction, MyAccount};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let payer = read_keypair_file("keypair.json")?;
    let client = Client::new(Cluster::Localnet, Rc::new(payer));

    // Create program
    let program = client.program(my_program::ID)?;

    // Send a transaction
    let my_account_kp = Keypair::new();
    program
        .request()
        .accounts(accounts::Initialize {
            my_account: my_account_kp.pubkey(),
            payer: program.payer(),
            system_program: system_program::ID,
        })
        .args(instruction::Initialize { field: 42 })
        .signer(&my_account_kp)
        .send()?;

    // Fetch an account
    let my_account: MyAccount = program.account(my_account_kp.pubkey())?;
    assert_eq!(my_account.field, 42);

    Ok(())
}
