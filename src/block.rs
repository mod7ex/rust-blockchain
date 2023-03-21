use std::time::SystemTime;
use crypto::{sha2::Sha256, digest::Digest};

use crate::error::TResult;
use crate::constants::TARGET_HEX;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Block {
    timestamp: u128,
    transaction: String,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32,
}

impl Block {
    pub fn new(
        data: String,
        prev_block_hash: String,
        height: usize
    ) -> TResult<Self> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        Ok(Block {
            timestamp,
            transaction: data,
            prev_block_hash,
            hash: String::from("empty"),
            height,
            nonce: 0
        })
    }

    pub fn hash_str(&self) -> &String {
        &self.hash
    }

    pub fn prev_hash_str(&self) -> &String {
        &self.prev_block_hash
    }

    fn prepare_hash_data(&self) -> TResult<Vec<u8>> {
        let content = (
            &self.prev_block_hash,
            &self.transaction,
            &self.timestamp,
            TARGET_HEX,
            &self.nonce
        );

        Ok(bincode::serialize(&content).unwrap())
    }

    fn hash_block(&self) -> TResult<String> {
        let mut hasher = Sha256::new();
        hasher.input(&self.prepare_hash_data().unwrap()[..]);
        Ok(hasher.result_str())
    }

    pub fn mine(&mut self) -> TResult<()> { // proof of work
        while !self.validate().unwrap() {
            self.nonce += 1;
        }

        self.hash = self.hash_block().unwrap();
        
        // println!("-------------------------------------------------");
        // println!("[Block mined]: {:#?}", self);
        // println!("-------------------------------------------------");

        Ok(())
    }

    fn validate(&self) -> TResult<bool> {
        let mut condition = vec![];
        condition.resize(TARGET_HEX, '0' as u8);

        let hash = self.hash_block().unwrap();

        Ok(&hash[0..TARGET_HEX] == String::from_utf8(condition).unwrap().as_str())
    }

    pub fn genesis_block() -> Self {
        Block::new(
            String::from("Genesis block"),
            String::new(),
            0,
        ).unwrap()
    }
}

