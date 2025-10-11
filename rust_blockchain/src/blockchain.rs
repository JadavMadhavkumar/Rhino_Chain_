use crate::block::Block;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, Read};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty,
        };
        blockchain.chain.push(Blockchain::create_genesis_block());
        blockchain
    }

    fn create_genesis_block() -> Block {
        let mut genesis_block = Block::new(0, "GENESIS_BLOCK", "0".to_string());
        genesis_block.mine_block(3); // Lower difficulty for genesis block
        genesis_block
    }

    pub fn add_block(&mut self, biometric_data: &str) {
        let previous_block = self.chain.last().unwrap().clone();
        let mut new_block = Block::new(
            previous_block.index + 1,
            biometric_data,
            previous_block.hash,
        );
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
            
            // Verify that biometric data hasn't been tampered with
            // In a real-world scenario, you would re-hash the actual biometric data
            // stored securely and compare with biometric_data_hash
        }
        true
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let blockchain: Blockchain = serde_json::from_str(&contents)?;
        Ok(blockchain)
    }
    
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
}