use crate::types::{Account, Hash};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub leaves: Vec<Account>,
    pub nodes: Vec<Hash>,
    pub root: Hash,
}

impl MerkleTree {
    pub fn new(accounts: Vec<Account>) -> Self {
        if accounts.is_empty() {
            return Self {
                leaves: vec![],
                nodes: vec![],
                root: [0u8; 32],
            };
        }

        let mut tree = Self {
            leaves: accounts.clone(),
            nodes: vec![],
            root: [0u8; 32],
        };

        tree.build();
        tree
    }

    fn build(&mut self) {
        if self.leaves.is_empty() {
            return;
        }

        // Start with leaf hashes
        let mut current_level: Vec<Hash> = self.leaves.iter().map(|acc| acc.hash()).collect();
        
        // Pad to next power of 2
        let target_size = next_power_of_2(current_level.len());
        while current_level.len() < target_size {
            current_level.push([0u8; 32]);
        }

        self.nodes = current_level.clone();

        // Build tree bottom-up
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in current_level.chunks(2) {
                let left = chunk[0];
                let right = if chunk.len() > 1 { chunk[1] } else { [0u8; 32] };
                let parent = hash_pair(&left, &right);
                next_level.push(parent);
            }
            
            self.nodes.extend(&next_level);
            current_level = next_level;
        }

        self.root = current_level[0];
    }

    pub fn get_proof(&self, account: &Account) -> Vec<Hash> {
        let leaf_index = self.leaves.iter().position(|acc| acc.address == account.address);
        
        match leaf_index {
            Some(index) => self.generate_proof(index),
            None => vec![],
        }
    }

    fn generate_proof(&self, leaf_index: usize) -> Vec<Hash> {
        let mut proof = Vec::new();
        let leaf_count = next_power_of_2(self.leaves.len());
        let mut index = leaf_index;
        let mut level_size = leaf_count;

        while level_size > 1 {
            let sibling_index = if index % 2 == 0 { index + 1 } else { index - 1 };
            
            if sibling_index < level_size {
                let node_offset = leaf_count - level_size;
                if sibling_index + node_offset < self.nodes.len() {
                    proof.push(self.nodes[sibling_index + node_offset]);
                } else {
                    proof.push([0u8; 32]);
                }
            } else {
                proof.push([0u8; 32]);
            }

            index /= 2;
            level_size /= 2;
        }

        proof
    }

    pub fn verify_proof(root: &Hash, account: &Account, proof: &[Hash]) -> bool {
        let mut current_hash = account.hash();
        
        for sibling in proof {
            current_hash = hash_pair(&current_hash, sibling);
        }

        &current_hash == root
    }

    pub fn update_account(&mut self, updated_account: Account) {
        if let Some(pos) = self.leaves.iter().position(|acc| acc.address == updated_account.address) {
            self.leaves[pos] = updated_account;
        } else {
            self.leaves.push(updated_account);
        }
        self.build();
    }

    pub fn update_accounts(&mut self, updated_accounts: Vec<Account>) {
        let mut account_map: HashMap<[u8; 20], Account> = HashMap::new();
        
        // Start with existing accounts
        for account in &self.leaves {
            account_map.insert(account.address, account.clone());
        }
        
        // Update with new account states
        for account in updated_accounts {
            account_map.insert(account.address, account);
        }
        
        self.leaves = account_map.into_values().collect();
        self.build();
    }
}

fn hash_pair(left: &Hash, right: &Hash) -> Hash {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Account;

    #[test]
    fn test_merkle_tree_creation() {
        let accounts = vec![
            Account::new([1u8; 20], 1000),
            Account::new([2u8; 20], 2000),
            Account::new([3u8; 20], 3000),
        ];

        let tree = MerkleTree::new(accounts);
        assert_ne!(tree.root, [0u8; 32]);
    }

    #[test]
    fn test_merkle_proof() {
        let account1 = Account::new([1u8; 20], 1000);
        let account2 = Account::new([2u8; 20], 2000);
        let accounts = vec![account1.clone(), account2];

        let tree = MerkleTree::new(accounts);
        let proof = tree.get_proof(&account1);
        
        assert!(MerkleTree::verify_proof(&tree.root, &account1, &proof));
    }
}