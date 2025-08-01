use serde::{Deserialize, Serialize, ser::Serializer, de::Deserializer};
use sha2::{Digest, Sha256};

pub type Hash = [u8; 32];
pub type Address = [u8; 20];



// Serialization functions for Address type
fn serialize_address<S>(address: &[u8; 20], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex_string = format!("0x{}", hex::encode(address));
    serializer.serialize_str(&hex_string)
}

fn deserialize_address<'de, D>(deserializer: D) -> Result<[u8; 20], D::Error>
where
    D: Deserializer<'de>,
{
    let hex_string = String::deserialize(deserializer)?;
    let hex_string = hex_string.trim_start_matches("0x");
    let bytes = hex::decode(hex_string).map_err(serde::de::Error::custom)?;
    if bytes.len() != 20 {
        return Err(serde::de::Error::custom("Address must be 20 bytes"));
    }
    let mut address = [0u8; 20];
    address.copy_from_slice(&bytes);
    Ok(address)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Account {
    /// The account address as hex string (0x...)
    #[serde(serialize_with = "serialize_address", deserialize_with = "deserialize_address")]
    pub address: Address,
    /// The account balance
    pub balance: u64,
    /// The account nonce (transaction counter)
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// The sender address as hex string (0x...)
    #[serde(serialize_with = "serialize_address", deserialize_with = "deserialize_address")]
    pub from: Address,
    /// The recipient address as hex string (0x...)
    #[serde(serialize_with = "serialize_address", deserialize_with = "deserialize_address")]
    pub to: Address,
    /// The transfer amount
    pub amount: u64,
    /// The transaction nonce
    pub nonce: u64,
    /// The transaction signature (64 bytes)
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