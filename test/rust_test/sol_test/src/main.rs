//generating a key pair-
use solana_sdk::{
    pubkey,
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    native_token::LAMPORTS_PER_SOL,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

mod accnt_type;
// #[tokio::main]
fn main() {
    //gen a new keypair
    let keypair = Keypair::new();
    //PDA
    let program_address = pubkey!("11111111111111111111111111111111");
    let seed = [b"adsince2k4".as_ref()];
    let (pda, bump) = Pubkey::find_program_address(&seed, &program_address);
    println!("Public Key: {}", keypair.pubkey());
    println!("Secret Key: {:?}", keypair.to_bytes());
    println!("BUMP: {:?}", bump);
    println!("PDA: {:?}", pda);

    //funding an accnt with SOL

}
