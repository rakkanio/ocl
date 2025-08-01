use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub address: [u8; 20],
    pub balance: u64,
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: [u8; 20],
    pub to: [u8; 20],
    pub amount: u64,
    pub nonce: u64,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchInput {
    pub prev_root: [u8; 32],
    pub transactions: Vec<Transaction>,
    pub accounts: Vec<Account>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOutput {
    pub new_root: [u8; 32],
    pub processed_count: u32,
}

impl Account {
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.address);
        hasher.update(&self.balance.to_le_bytes());
        hasher.update(&self.nonce.to_le_bytes());
        hasher.finalize().into()
    }
}

impl Transaction {
    pub fn is_valid(&self, from_account: &Account) -> bool {
        self.nonce == from_account.nonce + 1 
            && self.amount <= from_account.balance
            && self.from == from_account.address
    }
}

fn main() {
    // Read the input
    let input: BatchInput = env::read();
    
    // Create a map of accounts for easy lookup
    let mut account_map = std::collections::HashMap::new();
    for account in &input.accounts {
        account_map.insert(account.address, account.clone());
    }
    
    let mut processed_count = 0;
    
    // Process each transaction
    for tx in &input.transactions {
        if let Some(from_account) = account_map.get(&tx.from) {
            if tx.is_valid(from_account) {
                // Update from account
                let mut updated_from = from_account.clone();
                updated_from.balance -= tx.amount;
                updated_from.nonce += 1;
                account_map.insert(tx.from, updated_from);
                
                // Update to account
                let to_account = account_map.entry(tx.to).or_insert(Account {
                    address: tx.to,
                    balance: 0,
                    nonce: 0,
                });
                to_account.balance += tx.amount;
                
                processed_count += 1;
            }
        }
    }
    
    // Compute new merkle root
    let updated_accounts: Vec<Account> = account_map.values().cloned().collect();
    let new_root = compute_merkle_root(&updated_accounts);
    
    // Output the result
    let output = BatchOutput {
        new_root,
        processed_count,
    };
    
    env::commit(&output);
}

fn compute_merkle_root(accounts: &[Account]) -> [u8; 32] {
    if accounts.is_empty() {
        return [0u8; 32];
    }
    
    let mut hashes: Vec<[u8; 32]> = accounts.iter().map(|acc| acc.hash()).collect();
    
    // Pad to next power of 2
    let target_size = next_power_of_2(hashes.len());
    while hashes.len() < target_size {
        hashes.push([0u8; 32]);
    }
    
    // Build tree bottom-up
    while hashes.len() > 1 {
        let mut next_level = Vec::new();
        
        for chunk in hashes.chunks(2) {
            let left = chunk[0];
            let right = if chunk.len() > 1 { chunk[1] } else { [0u8; 32] };
            let parent = hash_pair(&left, &right);
            next_level.push(parent);
        }
        
        hashes = next_level;
    }
    
    hashes[0]
}

fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}

fn next_power_of_2(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut power = 1;
    while power < n {
        power *= 2;
    }
    power
} 