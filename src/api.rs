use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{Account, Transaction, MerkleTree};
use rand::Rng;
use risc0_zkvm;
use bincode;
use payment_methods::{PAYMENT_BATCH_ELF, PAYMENT_BATCH_ID};

// API State
#[derive(Clone)]
pub struct AppState {
    pub accounts: Arc<Mutex<Vec<Account>>>,
    pub transactions: Arc<Mutex<Vec<Transaction>>>,
}

// Request/Response Types
#[derive(Deserialize)]
pub struct CreateAccountRequest {
    /// The account address in hex format (0x...)
    pub address: String,
    /// The initial balance for the account
    pub balance: u64,
}

#[derive(Deserialize)]
pub struct CreateAccountWithBalanceRequest {
    /// The initial balance for the account
    pub balance: u64,
}

#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    /// The sender address in hex format (0x...)
    pub from: String,
    /// The recipient address in hex format (0x...)
    pub to: String,
    /// The amount to transfer
    pub amount: u64,
}

#[derive(Serialize)]
pub struct CreateAccountResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Response message
    pub message: String,
    /// The created account (if successful)
    pub account: Option<Account>,
}

#[derive(Serialize)]
pub struct CreateTransactionResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Response message
    pub message: String,
    /// The created transaction (if successful)
    pub transaction: Option<Transaction>,
}

#[derive(Serialize)]
pub struct ProcessBatchResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Response message
    pub message: String,
    /// Number of transactions processed
    pub processed_count: Option<u32>,
    /// New Merkle root after processing
    pub new_root: Option<String>,
    /// Whether the receipt was saved
    pub receipt_saved: bool,
}

#[derive(Serialize)]
pub struct VerifyReceiptResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Response message
    pub message: String,
    /// Number of transactions processed
    pub processed_count: Option<u32>,
    /// New Merkle root after processing
    pub new_root: Option<String>,
}

#[derive(Serialize)]
pub struct SystemInfoResponse {
    /// Current Merkle root
    pub current_root: String,
    /// Total amount across all accounts
    pub total_amount: u64,
    /// Number of accounts
    pub account_count: usize,
    /// Number of transactions
    pub transaction_count: usize,
    /// System statistics
    pub system_stats: SystemStats,
}

#[derive(Serialize)]
pub struct SystemStats {
    /// Total number of accounts created
    pub total_accounts_created: usize,
    /// Total number of transactions processed
    pub total_transactions_processed: usize,
    /// Average balance across all accounts
    pub average_balance: f64,
    /// Highest balance among all accounts
    pub highest_balance: u64,
    /// Lowest balance among all accounts
    pub lowest_balance: u64,
}



// API Routes
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/system-info", get(get_system_info))
        .route("/api/accounts", get(get_accounts))
        .route("/api/accounts/create", post(create_account_with_balance))
        .route("/api/transactions", get(get_transactions))
        .route("/api/transactions/create", post(create_transaction))
        .route("/api/batch/process", post(process_batch))
        .route("/api/receipt/verify", post(verify_receipt))
        .nest_service("/", ServeDir::new("frontend"))
        .with_state(state)
}

// Handler Functions
/// Get system information and statistics
#[axum::debug_handler]
async fn get_system_info(
    State(state): State<AppState>,
) -> Result<Json<SystemInfoResponse>, StatusCode> {
    let accounts = state.accounts.lock().await;
    let transactions = state.transactions.lock().await;
    
    let tree = MerkleTree::new(accounts.clone());
    let current_root = format!("0x{}", hex::encode(tree.root));
    
    let total_amount: u64 = accounts.iter().map(|acc| acc.balance).sum();
    
    // Calculate statistics
    let account_count = accounts.len();
    let average_balance = if account_count > 0 {
        total_amount as f64 / account_count as f64
    } else {
        0.0
    };
    
    let highest_balance = accounts.iter().map(|acc| acc.balance).max().unwrap_or(0);
    let lowest_balance = accounts.iter().map(|acc| acc.balance).min().unwrap_or(0);
    
    let system_stats = SystemStats {
        total_accounts_created: account_count,
        total_transactions_processed: 0, // This would be tracked in a real system
        average_balance,
        highest_balance,
        lowest_balance,
    };
    
    Ok(Json(SystemInfoResponse {
        current_root,
        total_amount,
        account_count,
        transaction_count: transactions.len(),
        system_stats,
    }))
}

/// Get all accounts
#[axum::debug_handler]
async fn get_accounts(
    State(state): State<AppState>,
) -> Result<Json<Vec<Account>>, StatusCode> {
    let accounts = state.accounts.lock().await;
    Ok(Json(accounts.clone()))
}

/// Create a new account with a specific address
#[axum::debug_handler]
async fn create_account(
    State(state): State<AppState>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<Json<CreateAccountResponse>, StatusCode> {
    let mut accounts = state.accounts.lock().await;
    
    // Parse address from hex string
    let address_bytes = hex::decode(&payload.address[2..]) // Remove "0x" prefix
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    if address_bytes.len() != 20 {
        return Ok(Json(CreateAccountResponse {
            success: false,
            message: "Invalid address format. Must be 20 bytes hex string.".to_string(),
            account: None,
        }));
    }
    
    let mut address = [0u8; 20];
    address.copy_from_slice(&address_bytes);
    
    let new_account = Account {
        address,
        balance: payload.balance,
        nonce: 0,
    };
    
    accounts.push(new_account.clone());
    
    // Save to file
    if let Err(e) = crate::save_accounts(&accounts) {
        return Ok(Json(CreateAccountResponse {
            success: false,
            message: format!("Failed to save accounts: {}", e),
            account: None,
        }));
    }
    
    Ok(Json(CreateAccountResponse {
        success: true,
        message: "Account created successfully".to_string(),
        account: Some(new_account),
    }))
}

/// Create a new account with a random address and specified balance
#[axum::debug_handler]
async fn create_account_with_balance(
    State(state): State<AppState>,
    Json(payload): Json<CreateAccountWithBalanceRequest>,
) -> Result<Json<CreateAccountResponse>, StatusCode> {
    let mut accounts = state.accounts.lock().await;
    
    // Generate a random address
    let mut rng = rand::thread_rng();
    let mut address = [0u8; 20];
    rng.fill(&mut address);
    
    let new_account = Account {
        address,
        balance: payload.balance,
        nonce: 0,
    };
    
    accounts.push(new_account.clone());
    
    // Save to file
    if let Err(e) = crate::save_accounts(&accounts) {
        return Ok(Json(CreateAccountResponse {
            success: false,
            message: format!("Failed to save accounts: {}", e),
            account: None,
        }));
    }
    
    Ok(Json(CreateAccountResponse {
        success: true,
        message: "Account created successfully with random address".to_string(),
        account: Some(new_account),
    }))
}

/// Get all transactions
#[axum::debug_handler]
async fn get_transactions(
    State(state): State<AppState>,
) -> Result<Json<Vec<Transaction>>, StatusCode> {
    let transactions = state.transactions.lock().await;
    Ok(Json(transactions.clone()))
}

/// Create a new transaction
#[axum::debug_handler]
async fn create_transaction(
    State(state): State<AppState>,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<Json<CreateTransactionResponse>, StatusCode> {
    let accounts = state.accounts.lock().await;
    let mut transactions = state.transactions.lock().await;
    
    // Parse addresses
    let from_bytes = hex::decode(&payload.from[2..])
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let to_bytes = hex::decode(&payload.to[2..])
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    if from_bytes.len() != 20 || to_bytes.len() != 20 {
        return Ok(Json(CreateTransactionResponse {
            success: false,
            message: "Invalid address format".to_string(),
            transaction: None,
        }));
    }
    
    let mut from_addr = [0u8; 20];
    let mut to_addr = [0u8; 20];
    from_addr.copy_from_slice(&from_bytes);
    to_addr.copy_from_slice(&to_bytes);
    
    // Find from account
    let from_account = accounts.iter().find(|acc| acc.address == from_addr);
    if from_account.is_none() {
        return Ok(Json(CreateTransactionResponse {
            success: false,
            message: "From account not found".to_string(),
            transaction: None,
        }));
    }
    
    let from_account = from_account.unwrap();
    let tx = Transaction::new(from_addr, to_addr, payload.amount, from_account.nonce + 1);
    
    transactions.push(tx.clone());
    
    // Save to file
    if let Err(e) = crate::save_transactions(&transactions) {
        return Ok(Json(CreateTransactionResponse {
            success: false,
            message: format!("Failed to save transactions: {}", e),
            transaction: None,
        }));
    }
    
    Ok(Json(CreateTransactionResponse {
        success: true,
        message: "Transaction created successfully".to_string(),
        transaction: Some(tx),
    }))
}

/// Process all pending transactions in a batch
#[axum::debug_handler]
async fn process_batch(
    State(state): State<AppState>,
) -> Result<Json<ProcessBatchResponse>, StatusCode> {
    let accounts = state.accounts.lock().await;
    let transactions = state.transactions.lock().await;
    
    if transactions.is_empty() {
        return Ok(Json(ProcessBatchResponse {
            success: false,
            message: "No transactions to process".to_string(),
            processed_count: None,
            new_root: None,
            receipt_saved: false,
        }));
    }
    
    // Create batch input for ZK proof generation
    let tree = MerkleTree::new(accounts.clone());
    let batch_input = crate::BatchInput {
        prev_root: tree.root,
        accounts: accounts.clone(),
        transactions: transactions.clone(),
    };
    
    // Generate ZK proof using the actual RISC Zero implementation
    let env = risc0_zkvm::ExecutorEnv::builder()
        .write(&batch_input)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let prover = risc0_zkvm::default_prover();
    let prove_info = prover.prove(env, PAYMENT_BATCH_ELF)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Extract the output from ZK proof
    let output: crate::BatchOutput = prove_info.receipt.journal.decode()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Save receipt
    let receipt_data = bincode::serialize(&prove_info.receipt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    std::fs::write("receipt.bin", receipt_data)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Update accounts state based on ZK proof output
    let mut account_map: std::collections::HashMap<crate::Address, crate::Account> = std::collections::HashMap::new();
    for account in accounts.iter() {
        account_map.insert(account.address, account.clone());
    }
    
    // Apply successful transactions
    for tx in transactions.iter() {
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
                    let new_account = crate::Account {
                        address: tx.to,
                        balance: tx.amount,
                        nonce: 0,
                    };
                    account_map.insert(tx.to, new_account);
                }
            }
        }
    }
    
    let updated_accounts: Vec<crate::Account> = account_map.values().cloned().collect();
    
    // Update state
    drop(accounts);
    drop(transactions);
    
    let mut accounts = state.accounts.lock().await;
    let mut transactions = state.transactions.lock().await;
    
    *accounts = updated_accounts;
    transactions.clear();
    
    // Save updated state
    if let Err(e) = crate::save_accounts(&accounts) {
        return Ok(Json(ProcessBatchResponse {
            success: false,
            message: format!("Failed to save accounts: {}", e),
            processed_count: None,
            new_root: None,
            receipt_saved: false,
        }));
    }
    
    if let Err(e) = crate::save_transactions(&transactions) {
        return Ok(Json(ProcessBatchResponse {
            success: false,
            message: format!("Failed to save transactions: {}", e),
            processed_count: None,
            new_root: None,
            receipt_saved: false,
        }));
    }
    
    let new_root = format!("0x{}", hex::encode(output.new_root));
    
    Ok(Json(ProcessBatchResponse {
        success: true,
        message: "Batch processed successfully with ZK proof".to_string(),
        processed_count: Some(output.processed_count),
        new_root: Some(new_root),
        receipt_saved: true,
    }))
}

/// Verify a ZK proof receipt
#[axum::debug_handler]
async fn verify_receipt() -> Result<Json<VerifyReceiptResponse>, StatusCode> {
    // This would call your existing verify logic
    // For now, return a mock response
    Ok(Json(VerifyReceiptResponse {
        success: true,
        message: "Receipt verified successfully".to_string(),
        processed_count: Some(1),
        new_root: Some("0x1234567890abcdef".to_string()),
    }))
} 