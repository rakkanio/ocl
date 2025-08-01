use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub type Hash = [u8; 32];
pub type Address = [u8; 20];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Account {
    pub address: Address,
    pub balance: u64,
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub nonce: u64,
    pub signature: Vec<u8>, // Simplified signature
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchInput {
    pub prev_root: Hash,
    pub transactions: Vec<Transaction>,
    pub accounts: Vec<Account>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOutput {
    pub new_root: Hash,
    pub processed_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalProof {
    pub account: Account,
    pub merkle_proof: Vec<Hash>,
    pub receipt: Vec<u8>,
}

impl Account {
    pub fn new(address: Address, balance: u64) -> Self {
        Self {
            address,
            balance,
            nonce: 0,
        }
    }

    pub fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(&self.address);
        hasher.update(&self.balance.to_le_bytes());
        hasher.update(&self.nonce.to_le_bytes());
        hasher.finalize().into()
    }
}

impl Transaction {
    pub fn new(from: Address, to: Address, amount: u64, nonce: u64) -> Self {
        Self {
            from,
            to,
            amount,
            nonce,
            signature: vec![0u8; 64], // Simplified - in real implementation, sign the tx
        }
    }

    pub fn is_valid(&self, from_account: &Account) -> bool {
        // Basic validation
        self.nonce == from_account.nonce + 1 
            && self.amount <= from_account.balance
            && self.from == from_account.address
    }
}