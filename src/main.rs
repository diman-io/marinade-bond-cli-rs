use solana_sdk::system_instruction;
use {
    borsh::{BorshDeserialize, BorshSerialize},
    clap::Parser,
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        signer::EncodableKey,
        transaction::Transaction,
    },
};

#[derive(Debug, Parser)]
struct ClapArgs {
    #[arg(long, value_name = &"RPC_URL")]
    url: String,

    #[arg(long, value_name = &"KEYPAIR")]
    fee_payer: String,

    #[arg(long, value_name = &"KEYPAIR")]
    rent_payer: String,

    #[arg(long, value_name = &"PUBKEY")]
    identity_address: Pubkey,

    #[arg(long, value_name = &"PUBKEY")]
    vote_address: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct BondInitInstructionData {
    operation_id: [u8; 8],
    bond_authority: Pubkey,
    revenue_share: u32,
}

impl BondInitInstructionData {
    fn new(bond_authority: Pubkey) -> Self {
        Self {
            operation_id: [0x5f, 0x5d, 0x5d, 0xb5, 0xdd, 0x24, 0x7e, 0x40], // fucking anchor
            bond_authority,
            revenue_share: 0,
        }
    }
}

const BOND_PROGRAM: Pubkey = solana_sdk::pubkey!("vBoNdEvzMrSai7is21XgVYik65mqtaKXuSdMBJ1xkW4");
const BOND_CONFIG: Pubkey = solana_sdk::pubkey!("vbMaRfmTCg92HWGzmd53APkMNpPnGVGZTUHwUJQkXAU");
const BOND_SEED: &[u8] = b"bond_account";

fn main() {
    let args = ClapArgs::parse();

    let client = RpcClient::new(args.url);

    let fee_payer = Keypair::read_from_file(args.fee_payer).unwrap();
    let rent_payer = Keypair::read_from_file(args.rent_payer).unwrap();

    let instruction_data = BondInitInstructionData::new(args.identity_address);

    let serialized_instruction_data = instruction_data.try_to_vec().unwrap();

    let bond_address = Pubkey::find_program_address(
        &[BOND_SEED, BOND_CONFIG.as_ref(), args.vote_address.as_ref()],
        &BOND_PROGRAM,
    )
    .0;

    const RENT_AMOUNT: u64 = 002700480;

    let fund_instruction =
        system_instruction::transfer(&fee_payer.pubkey(), &rent_payer.pubkey(), RENT_AMOUNT);

    let bond_instruction = Instruction {
        program_id: BOND_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(BOND_CONFIG, false),
            AccountMeta::new_readonly(args.vote_address, false),
            AccountMeta::new_readonly(BOND_PROGRAM, false),
            AccountMeta::new(bond_address, false),
            AccountMeta::new(rent_payer.pubkey(), true),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data: serialized_instruction_data,
    };

    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get recent block hash");
    let transaction = Transaction::new_signed_with_payer(
        &[fund_instruction, bond_instruction],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &rent_payer],
        recent_blockhash,
    );

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction sent successfully. Signature: {}", signature),
        Err(e) => {
            eprintln!("Error sending transaction: {}", e);
            match client.simulate_transaction(&transaction) {
                Ok(result) => {
                    eprintln!("Transaction: {:#?}", transaction);
                    eprintln!("Transaction simulated successfully: {:#?}", result)
                }
                Err(e) => eprintln!("Error simulating transaction: {}", e),
            }
        }
    }
}
