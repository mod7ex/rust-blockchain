use clap::{Command, arg};

use crate::{error::TResult, block_chain::BlockChain};

pub struct Cli {
    block_chain: BlockChain
}

impl Cli {
    pub fn new() -> TResult<Self> {
        Ok(Cli {
            block_chain: BlockChain::new().unwrap()
        })
    }

    pub fn run(&mut self) -> TResult<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("example@mail.com")
            .about("blockchain in rust: simple example to have a basic idea")
            .subcommand(
                Command::new("print-chain")
                            .about("prints all the chain blocks")
            )
            .subcommand(
                Command::new("add-block")
                            .about("adds a block to the blockchain")
                            .arg(arg!(<DATA>" 'the block data'"))
            )
            .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("add-block") {
            if let Some(c) = matches.get_one::<String>("DATA") {
                self.block_chain.add_block(c).unwrap()
            } else {
                println!("Not printing testing lists ...");
            }
        }

        if let Some(_) = matches.subcommand_matches("print-chain") {
            self.prit_chain()
        }

        Ok(())
    }

    fn prit_chain(&mut self) {
        for block in self.block_chain.iter() {
            println!("{:#?}", block);
        }
    }
}