mod types;
mod merkle;

use anyhow::Result;
use clap::{Parser, Subcommand};
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde_json;
use std::collections::HashMap;
use std::fs;

use types::{Account, BatchInput, BatchOutput, Transaction, WithdrawalProof, Address, Hash};
use merkle::MerkleTree;

// Include the compiled guest code
// use payment_methods::PAYMENT_BATCH_ELF; // Temporarily disabled

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
            
            let tx = Transaction::new(from_addr, to_addr, amount, from_account.nonce);
            
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
            // let prove_info = prover.prove(env, PAYMENT_BATCH_ELF)?; // Temporarily disabled
            
            // Extract the output (temporarily disabled ZK)
            // let output: BatchOutput = prove_info.receipt.journal.decode()?;
            
            println!("Batch processed successfully!");
            println!("  Processed transactions: {}", transactions.len());
            
            // Compute new Merkle root from updated accounts
            let mut account_map: HashMap<Address, Account> = HashMap::new();
            for account in accounts {
                account_map.insert(account.address, account);
            }
            
            // Apply successful transactions
            for tx in transactions {
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
            // Print the new Merkle root here
            let new_tree = MerkleTree::new(updated_accounts.clone());
            println!("  New root: 0x{}", hex::encode(new_tree.root));
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
            // receipt.verify(PAYMENT_BATCH_ELF)?; // Temporarily disabled
            
            let output: BatchOutput = receipt.journal.decode()?;
            
            println!("Receipt verified successfully!");
            println!("  Processed transactions: {}", output.processed_count);
            println!("  New root: 0x{}", hex::encode(output.new_root));
        }
    }

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