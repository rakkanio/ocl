mod types;
mod merkle;
mod api;

use anyhow::Result;
use clap::{Parser, Subcommand};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{
    routing::Router,
    http::Method,
};

use types::{Account, BatchInput, BatchOutput, Transaction, WithdrawalProof, Address, Hash};
use merkle::MerkleTree;

// Include the compiled guest code
use payment_methods::{PAYMENT_BATCH_ELF, PAYMENT_BATCH_ID};

#[derive(Parser)]
#[command(name = "risc-zero-payment")]
#[command(about = "A RISC Zero payment system with Merkle trees")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize with sample accounts
    Init {
        #[arg(short, long, default_value = "accounts.json")]
        file: String,
    },
    /// Create a transaction
    CreateTx {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        amount: u64,
    },
    /// Process a batch of transactions
    ProcessBatch {
        #[arg(short, long, default_value = "accounts.json")]
        accounts_file: String,
        #[arg(short, long, default_value = "transactions.json")]
        tx_file: String,
    },
    /// Verify a receipt
    Verify {
        #[arg(short, long, default_value = "receipt.bin")]
        receipt_file: String,
    },
    /// Start the API server
    Serve {
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
}

fn parse_address(addr_str: &str) -> Result<Address> {
    let bytes = hex::decode(addr_str.trim_start_matches("0x"))?;
    if bytes.len() != 20 {
        return Err(anyhow::anyhow!("Address must be 20 bytes"));
    }
    let mut addr = [0u8; 20];
    addr.copy_from_slice(&bytes);
    Ok(addr)
}

fn address_to_string(addr: &Address) -> String {
    format!("0x{}", hex::encode(addr))
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { file } => {
            println!("Initializing with sample accounts...");
            
            let accounts = vec![
                Account::new(parse_address("0x1111111111111111111111111111111111111111")?, 10000),
                Account::new(parse_address("0x2222222222222222222222222222222222222222")?, 5000),
                Account::new(parse_address("0x3333333333333333333333333333333333333333")?, 7500),
            ];

            let json = serde_json::to_string_pretty(&accounts)?;
            fs::write(&file, json)?;
            
            let tree = MerkleTree::new(accounts.clone());
            println!("Initial Merkle root: 0x{}", hex::encode(tree.root));
            println!("Accounts saved to {}", file);
            
            // Also save empty transactions file
            let empty_txs: Vec<Transaction> = vec![];
            let tx_json = serde_json::to_string_pretty(&empty_txs)?;
            fs::write("transactions.json", tx_json)?;
            println!("Empty transactions file created: transactions.json");
        }

        Commands::CreateTx { from, to, amount } => {
            println!("Creating transaction...");
            
            let from_addr = parse_address(&from)?;
            let to_addr = parse_address(&to)?;
            
            // Load existing accounts to get nonce
            let accounts_data = fs::read_to_string("accounts.json")?;
            let accounts: Vec<Account> = serde_json::from_str(&accounts_data)?;
            
            let from_account = accounts.iter()
                .find(|acc| acc.address == from_addr)
                .ok_or_else(|| anyhow::anyhow!("From account not found"))?;
            
            let tx = Transaction::new(from_addr, to_addr, amount, from_account.nonce + 1);
            
            // Load existing transactions
            let tx_data = fs::read_to_string("transactions.json").unwrap_or_else(|_| "[]".to_string());
            let mut transactions: Vec<Transaction> = serde_json::from_str(&tx_data)?;
            
            transactions.push(tx.clone());
            
            let tx_json = serde_json::to_string_pretty(&transactions)?;
            fs::write("transactions.json", tx_json)?;
            
            println!("Transaction created:");
            println!("  From: {}", address_to_string(&tx.from));
            println!("  To: {}", address_to_string(&tx.to));
            println!("  Amount: {}", tx.amount);
            println!("  Nonce: {}", tx.nonce);
        }

        Commands::ProcessBatch { accounts_file, tx_file } => {
            println!("Processing transaction batch...");
            
            // Load accounts and transactions
            let accounts_data = fs::read_to_string(&accounts_file)?;
            let accounts: Vec<Account> = serde_json::from_str(&accounts_data)?;
            
            let tx_data = fs::read_to_string(&tx_file)?;
            let transactions: Vec<Transaction> = serde_json::from_str(&tx_data)?;
            
            if transactions.is_empty() {
                println!("No transactions to process");
                return Ok(());
            }
            
            // Compute initial root
            let initial_tree = MerkleTree::new(accounts.clone());
            println!("Initial root: 0x{}", hex::encode(initial_tree.root));
            
            // Prepare input for guest
            let batch_input = BatchInput {
                prev_root: initial_tree.root,
                transactions: transactions.clone(),
                accounts: accounts.clone(),
            };
            
            // Set up the executor environment
            let env = ExecutorEnv::builder()
                .write(&batch_input)?
                .build()?;
            
            // Run the prover
            println!("Generating proof...");
            let prover = default_prover();
            let prove_info = prover.prove(env, PAYMENT_BATCH_ELF)?;
            
            // Extract the output from ZK proof
            let output: BatchOutput = prove_info.receipt.journal.decode()?;
            
            println!("Batch processed successfully!");
            println!("  Processed transactions: {}", output.processed_count);
            println!("  New root: 0x{}", hex::encode(output.new_root));
            
            // Save receipt
            let receipt_data = bincode::serialize(&prove_info.receipt).map_err(|e| anyhow::anyhow!("Failed to serialize receipt: {}", e))?;
            fs::write("receipt.bin", receipt_data)?;
            println!("Receipt saved to receipt.bin");
            
            // Update accounts state based on ZK proof output
            let mut account_map: HashMap<Address, Account> = HashMap::new();
            for account in &accounts {
                account_map.insert(account.address, account.clone());
            }
            
            // Apply successful transactions
            for tx in &transactions {
                if let Some(from_account) = account_map.get(&tx.from).cloned() {
                    if tx.is_valid(&from_account) {
                        let mut updated_from = from_account;
                        updated_from.balance -= tx.amount;
                        updated_from.nonce += 1;
                        account_map.insert(tx.from, updated_from);
                        
                        if let Some(mut to_account) = account_map.get(&tx.to).cloned() {
                            to_account.balance += tx.amount;
                            account_map.insert(tx.to, to_account);
                        } else {
                            let new_account = Account {
                                address: tx.to,
                                balance: tx.amount,
                                nonce: 0,
                            };
                            account_map.insert(tx.to, new_account);
                        }
                    }
                }
            }
            
            let updated_accounts: Vec<Account> = account_map.values().cloned().collect();
            

            let updated_json = serde_json::to_string_pretty(&updated_accounts)?;
            fs::write(&accounts_file, updated_json)?;
            println!("Updated accounts saved to {}", accounts_file);
            
            // Clear processed transactions
            fs::write(&tx_file, "[]")?;
        }

        Commands::Verify { receipt_file } => {
            println!("Verifying receipt...");
            
            let receipt_data = fs::read(&receipt_file)?;
            let receipt: risc0_zkvm::Receipt = bincode::deserialize(&receipt_data).map_err(|e| anyhow::anyhow!("Failed to deserialize receipt: {}", e))?;
            
            // Verify the receipt
            receipt.verify(PAYMENT_BATCH_ID)?;
            
            let output: BatchOutput = receipt.journal.decode()?;
            
            println!("Receipt verified successfully!");
            println!("  Processed transactions: {}", output.processed_count);
            println!("  New root: 0x{}", hex::encode(output.new_root));
        }

        Commands::Serve { port } => {
            // Use PORT environment variable for Render, fallback to provided port
            let port = std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse::<u16>().ok())
                .unwrap_or(port);
            
            println!("Starting API server on port {}...", port);
            
            // Load initial state
            let accounts = load_accounts("accounts.json").unwrap_or_else(|_| vec![]);
            let transactions = load_transactions("transactions.json").unwrap_or_else(|_| vec![]);
            
            // Create app state
            let state = api::AppState {
                accounts: Arc::new(Mutex::new(accounts)),
                transactions: Arc::new(Mutex::new(transactions)),
            };
            
            let app = api::create_router(state);
            
            // Start server - bind to 0.0.0.0 for Render
            let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
            println!("API server running on port {}", port);
            println!("Available endpoints:");
            println!("  GET  /api/system-info - System information");
            println!("  GET  /api/accounts - List accounts");
            println!("  POST /api/accounts/create - Create account");
            println!("  GET  /api/transactions - List transactions");
            println!("  POST /api/transactions/create - Create transaction");
            println!("  POST /api/batch/process - Process batch");
            println!("  POST /api/receipt/verify - Verify receipt");
            
            axum::serve(listener, app).await?;
        }
    }

    Ok(())
}

// Helper functions for loading/saving data
pub fn load_accounts(filename: &str) -> Result<Vec<Account>> {
    if !std::path::Path::new(filename).exists() {
        return Ok(vec![]);
    }
    let content = fs::read_to_string(filename)?;
    Ok(serde_json::from_str(&content)?)
}

pub fn load_transactions(filename: &str) -> Result<Vec<Transaction>> {
    if !std::path::Path::new(filename).exists() {
        return Ok(vec![]);
    }
    let content = fs::read_to_string(filename)?;
    Ok(serde_json::from_str(&content)?)
}

pub fn save_accounts(accounts: &[Account]) -> Result<()> {
    let json = serde_json::to_string_pretty(accounts)?;
    fs::write("accounts.json", json)?;
    Ok(())
}

pub fn save_transactions(transactions: &[Transaction]) -> Result<()> {
    let json = serde_json::to_string_pretty(transactions)?;
    fs::write("transactions.json", json)?;
    Ok(())
}

// Helper function for bincode (add this dependency to Cargo.toml if not present)
mod bincode {
    use serde::{Deserialize, Serialize};
    
    pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(serde_json::to_vec(value)?)
    }
    
    pub fn deserialize<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, Box<dyn std::error::Error>> {
        Ok(serde_json::from_slice(bytes)?)
    }
}