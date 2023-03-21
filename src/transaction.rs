use crypto::{sha2::Sha256, digest::Digest};

use crate::error::TResult;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub id: String,
    pub v_in: Vec<TXInput>,
    pub v_out: Vec<TXOutput>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TXInput {
    pub tx_id: String,
    pub v_out: i32,
    pub script_sig: String
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TXOutput {
    pub value: i32,
    pub script_pub_key: String
}

impl Transaction {
    pub fn new_coinbase(to: String, mut data: String) -> TResult<Self> {
        
        if data.is_empty() {
            data = format!("Reward to '{}'", to);
        }

        let mut tx = Transaction {
            id: String::new(),
            v_in: vec![TXInput {
                    tx_id: String::new(),
                    v_out: -1,
                    script_sig: data
                }],
            v_out: vec![TXOutput {
                    value: 100,
                    script_pub_key: to
                }]
        };

        tx.set_id().unwrap();

        Ok(tx)
    }

    fn set_id(&mut self) -> TResult<()> {
        let mut hasher = Sha256::new();
        let data = bincode::serialize(&self).unwrap();
        hasher.input(&data);
        self.id = hasher.result_str();
        Ok(())
    }

    pub fn is_coinbase(&self) -> bool {
        self.v_in.len() == 1 && self.v_in[0].tx_id.is_empty() && self.v_in[0].v_out == -1
    }
}

impl TXInput {
    pub fn can_unlock_output_with(&self, unlocking_data: &str) -> bool {
        self.script_sig == unlocking_data
    }
}

impl TXOutput {
    pub fn can_be_unlock_with(&self, unlocking_data: &str) -> bool {
        self.script_pub_key == unlocking_data
    }
}