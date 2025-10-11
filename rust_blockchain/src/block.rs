use chrono::{Utc, DateTime};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: String,
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub biometric_data_hash: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, biometric_data: &str, previous_hash: String) -> Self {
        let timestamp = Utc::now();
        let biometric_data_hash = Block::hash_biometric_data(biometric_data);
        let mut block = Block {
            id: Uuid::new_v4().to_string(),
            index,
            timestamp,
            biometric_data_hash,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}{}{}",
            self.id, self.index, self.timestamp, self.biometric_data_hash, self.previous_hash, self.nonce
        ));
        format!("{:x}", hasher.finalize())
    }

    pub fn hash_biometric_data(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}