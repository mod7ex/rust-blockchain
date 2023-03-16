use std::time::{SystemTime, SystemTimeError};
use crypto::{sha2::Sha256, digest::Digest};
use log::info;

type TResult<T> = Result<T, failure::Error>;

static TARGET_HEX: usize = 4;

#[derive(Debug, Clone)]
pub struct Block {
    timestamp: u128,
    transaction: String,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32,
}

#[derive(Debug, Clone)]
pub struct BlockChain {
    blocks: Vec<Block>
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
            hash: String::new(),
            height,
            nonce: 0
        })
    }

    fn mine(&mut self) -> TResult<()> { // proof of work
        info!("Mining the block");

        while !self.validate().unwrap() {
            self.nonce += 1;
        }

        let data = self.prepare_hash_data().unwrap();
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    fn validate(&self) -> TResult<bool> {
        let data = self.prepare_hash_data().unwrap();
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);

        let mut condition = vec![];
        condition.resize(TARGET_HEX, '0' as u8);
        println!("{:?}", condition);
        Ok(&hasher.result_str()[0..TARGET_HEX] == String::from_utf8(condition).unwrap().as_str())
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

    fn genesis_block() -> Self {
        Block::new(
            String::from("Genesis block"),
            String::new(),
            0,
        ).unwrap()
    }
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: vec![Block::genesis_block()]
        }
    }

    pub fn add_block(&mut self, data: String) -> TResult<()> {
        let prev = self.blocks.last().unwrap();

        self.blocks.push(Block::new(
            data,
            prev.hash.clone(),
            TARGET_HEX
        ).unwrap());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain() {
        let mut b = BlockChain::new();
        println!("{:#?}", b);
        b.add_block("a".to_string()).unwrap();
        b.add_block("b".to_string()).unwrap();
        b.add_block("c".to_string()).unwrap();
        println!("{:#?}", b);
    }
}