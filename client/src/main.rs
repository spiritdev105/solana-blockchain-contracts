use std::str::FromStr;

// use borsh::{BorshDeserialize, BorshSerialize};
use solana_cli_config::Config;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::AccountMeta, message::Message, pubkey::Pubkey, signer::Signer, system_program,
    transaction::Transaction,
};

// const INITIALIZE_DISCRIMINANT: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];

// #[derive(BorshSerialize, BorshDeserialize)]
// pub struct UserInfoParams {
//     pub name: String,
//     pub age: u8,
// }

const PROGRAM_ID: &str = "ARbowygvyWvish2HNEwsGexXtFHy1ujMAk36sjdSBRrN";

fn main() {
    // let user = UserInfoParams {
    //     name: "Alice".to_string(),
    //     age: 30,
    // };

    let config_file = solana_cli_config::CONFIG_FILE
        .as_ref()
        .expect("unable to get config file path");
    let cli_config: Config = Config::load(config_file).expect("Unable to load config file");

    let connection = RpcClient::new(cli_config.json_rpc_url);

    let signer = solana_clap_utils::keypair::keypair_from_path(
        &Default::default(),
        &cli_config.keypair_path,
        "keypair",
        false,
    )
    .map_err(|err| println!("Unable to get signer from path: {}", err))
    .unwrap();
    let signer_pubkey = signer.pubkey();

    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();

    let seeds: &[&[u8]; 2] = &[b"hello", &signer_pubkey.to_bytes()];

    let (pda_account, _) = Pubkey::find_program_address(seeds, &program_id);

    // let ix = solana_sdk::instruction::Instruction::new_with_borsh(
    //     program_id,
    //     &(INITIALIZE_DISCRIMINANT, user),
    //     vec![
    //         AccountMeta::new(pda_account, false),
    //         AccountMeta::new_readonly(signer_pubkey, true),
    //         AccountMeta::new_readonly(system_program::ID, false),
    //     ],
    // );

    let data: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
    let ix = solana_sdk::instruction::Instruction::new_with_bytes(
        program_id,
        &data,
        vec![
            AccountMeta::new(pda_account, false),
            AccountMeta::new_readonly(signer_pubkey, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
    );
    let message = Message::new(&[ix], Some(&signer_pubkey));

    let mut tx = Transaction::new_unsigned(message);

    tx.sign(&[&signer], connection.get_latest_blockhash().unwrap());

    let tx_id = connection
        .send_and_confirm_transaction_with_spinner(&tx)
        .map_err(|err| {
            println!("{:?}", err);
        })
        .unwrap();
    println!("Program uploaded successfully. Transaction ID: {}", tx_id);
}
