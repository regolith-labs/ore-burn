use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    signature::{read_keypair_file, Signer},
    transaction::Transaction,
};

#[tokio::main]
async fn main() {
    // Read keypair from file
    let payer =
        read_keypair_file(&std::env::var("KEYPAIR").expect("Missing KEYPAIR env var")).unwrap();

    // Build transaction
    let rpc = RpcClient::new(std::env::var("RPC").expect("Missing RPC env var"));
    let cu_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);
    // let initialize_ix = ore_bury_api::sdk::initialize(payer.pubkey());
    let bury_ix = ore_bury_api::sdk::bury(payer.pubkey());
    let blockhash = rpc.get_latest_blockhash().await.unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[cu_budget_ix, bury_ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    // // Send transaction
    match rpc.send_and_confirm_transaction(&transaction).await {
        Ok(signature) => println!("Transaction succeeded! Signature: {}", signature),
        Err(err) => println!("Transaction failed: {}", err),
    }
}
