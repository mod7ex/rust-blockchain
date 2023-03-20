use crate::block::Block;

use crate::error::TResult;

#[derive(Debug)]
pub struct BlockChain {
    current_hash: String,
    db: sled::Db,
}

pub struct BlockChainIter<'a> {
    current_hash: String,
    block_chain: &'a BlockChain
}

impl BlockChain {
    pub fn new() -> TResult<Self> {

        let db = sled::open("data/blocks").unwrap();

        match db.get("LAST").unwrap() {
            Some(hash) => {
                let current_hash = String::from_utf8(hash.to_vec()).unwrap();

                Ok(BlockChain {
                    db,
                    current_hash,
                })
            }
            None => {
                let mut block = Block::genesis_block();

                block.mine().unwrap();
                
                let current_hash = block.hash_str().clone();

                db.insert(&current_hash, bincode::serialize(&block).unwrap()).unwrap();
                db.insert("LAST", current_hash.as_bytes()).unwrap();
                db.insert("HEIGHT", &(0_i32).to_ne_bytes()).unwrap();

                let block_chain = BlockChain {
                    db,
                    current_hash,
                };

                block_chain.db.flush().unwrap();

                Ok(block_chain)
            }
        }
    }

    pub fn add_block(&mut self, data: &str) -> TResult<()> {
        /* let current_hash = String::from_utf8(self.db.get("LAST").unwrap().unwrap().to_vec()).unwrap(); */
        let current_hash = self.current_hash.clone();

        let encoded_height = &self.db.get("HEIGHT").unwrap().unwrap().to_vec();
        let height: usize = i32::from_ne_bytes(encoded_height[..4].try_into().unwrap()).try_into().unwrap();

        let mut new_block = Block::new(
            data.to_string(),
            current_hash,
            height + 1
        ).unwrap();

        new_block.mine().unwrap();

        let new_block_hash = new_block.hash_str();

        self.db.insert(&new_block_hash, bincode::serialize(&new_block).unwrap()).unwrap();
        self.db.insert("LAST", new_block_hash.as_bytes()).unwrap();
        self.db.insert("HEIGHT", &(height + 1).to_ne_bytes()).unwrap();

        self.current_hash = new_block_hash.clone();

        Ok(())
    }

    pub fn iter(&self) -> BlockChainIter {
        BlockChainIter {
            current_hash: self.current_hash.clone(),
            block_chain: &self
        }
    }
}

impl<'a> Iterator for BlockChainIter<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encoded_block) = self.block_chain.db.get(&self.current_hash) {
            return match encoded_block {
                Some(serialized_block) => {
                    if let Ok(block) = bincode::deserialize::<Block>(&serialized_block) {
                        self.current_hash = block.prev_hash_str().clone();

                        Some(block)
                    } else {
                        None
                    }
                }
                None => None
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_block() {
        let mut b = BlockChain::new().unwrap();

        b.add_block("data 1").unwrap();
        b.add_block("data 2").unwrap();
        b.add_block("data 3").unwrap();

        for block in b.iter() {
            println!("{:#?}", block);
        }
    }
}