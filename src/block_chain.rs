use std::collections::HashMap;

use crate::block::Block;

use crate::error::TResult;
use crate::transaction::{Transaction, TXOutput};

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

        let hash = db
            .get("LAST")
            .unwrap()
            .expect("A new block should be created first");

        Ok(BlockChain {
            db,
            current_hash: String::from_utf8(hash.to_vec()).unwrap(),
        })
    }

    pub fn create_blockchain(address: String) -> TResult<BlockChain> {
        let db = sled::open("data/blocks").unwrap();

        let coinbase_tx = Transaction::new_coinbase(address, String::from("GENESIS_COINBASE_TX")).unwrap();

        let mut block = Block::genesis_block(coinbase_tx);

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

    pub fn add_block(&mut self, data: Vec<Transaction>) -> TResult<()> {
        /* let current_hash = String::from_utf8(self.db.get("LAST").unwrap().unwrap().to_vec()).unwrap(); */
        let current_hash = self.current_hash.clone();

        let encoded_height = &self.db.get("HEIGHT").unwrap().unwrap().to_vec();
        let height: usize = i32::from_ne_bytes(encoded_height[..4].try_into().unwrap()).try_into().unwrap();

        let mut new_block = Block::new(
            data,
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

    fn find_unspent_transactions(&self, addr: &str) -> Vec<Transaction> {
        let mut spent_TXOs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut unspent_TXs = Vec::new();

        for block in self.iter() {
            for tx in block.transactions() {
                for index in 0..tx.v_out.len() {
                    if let Some(ids) = spent_TXOs.get(&tx.id) {
                        if ids.contains(&(index as i32)) {
                            continue;
                        }
                    }

                    if tx.v_out[index].can_be_unlock_with(addr) {
                        unspent_TXs.push(tx.clone());
                    }
                }

                if !tx.is_coinbase() {
                    for i in &tx.v_in {
                        if i.can_unlock_output_with(addr) {
                            match spent_TXOs.get_mut(&i.tx_id) {
                                Some(v) => {
                                    v.push(i.v_out);
                                }
                                None => {
                                    spent_TXOs.insert(
                                      i.tx_id.clone(),
                                      vec![i.v_out]  
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        unspent_TXs
    }

    pub fn find_UTXo(&self, addr: &str) -> Vec<TXOutput> {
        let mut utxos = Vec::<TXOutput>::new();
        let unspend_TXs = self.find_unspent_transactions(addr);
        for tx in unspend_TXs {
            for out in &tx.v_out {
                if out.can_be_unlock_with(addr) {
                    utxos.push(out.clone());
                }
            }
        }

        utxos
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
/*
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
*/