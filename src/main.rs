mod block;
mod error;
mod block_chain;
mod constants;
mod cli;
mod transaction;

fn main() {
    let mut cli = cli::Cli::new().unwrap();

    cli.run().unwrap();
}