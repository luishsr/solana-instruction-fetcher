use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_transaction_status::{UiTransactionEncoding, option_serializer::OptionSerializer};
use solana_client::rpc_config::RpcTransactionConfig;
use std::io::{self, Write};
use std::str::FromStr;

fn main() {
    // Define Solana mainnet RPC URL
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    // Get transaction signature from user input
    print!("Enter transaction signature: ");
    io::stdout().flush().unwrap();
    let mut tx_signature = String::new();
    io::stdin().read_line(&mut tx_signature).unwrap();
    let tx_signature = tx_signature.trim();

    // Parse the transaction signature
    let signature = match Signature::from_str(tx_signature) {
        Ok(sig) => sig,
        Err(_) => {
            eprintln!("Invalid transaction signature format.");
            return;
        }
    };

    // Get the transaction details with maxSupportedTransactionVersion set to 0
    let config = RpcTransactionConfig {
        encoding: Option::from(UiTransactionEncoding::Json),
        max_supported_transaction_version: Some(0),
        ..RpcTransactionConfig::default()
    };

    match client.get_transaction_with_config(&signature, config) {
        Ok(txn) => {
            println!("Transaction found. Listing details:");

            // Access the transaction meta
            if let Some(meta) = txn.transaction.meta {
                // Print fee information
                println!("Fee: {} lamports", meta.fee);

                // Print pre and post balances
                println!("Pre Balances: {:?}", meta.pre_balances);
                println!("Post Balances: {:?}", meta.post_balances);

                // Print log messages if available
                if let OptionSerializer::Some(log_messages) = &meta.log_messages {
                    println!("Log Messages:");
                    for (index, log) in log_messages.iter().enumerate() {
                        println!("  [{}]: {}", index + 1, log);
                    }
                }

                // Print inner instructions if available
                if let OptionSerializer::Some(inner_instructions) = &meta.inner_instructions {
                    println!("Inner Instructions:");
                    for (index, instruction) in inner_instructions.iter().enumerate() {
                        println!("  [{}]: {:?}", index + 1, instruction);
                    }
                }
            } else {
                println!("No transaction metadata available.");
            }
        },
        Err(err) => {
            eprintln!("Failed to get transaction details: {}", err);
        }
    }
}
