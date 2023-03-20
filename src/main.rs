mod block;
mod error;
mod block_chain;
mod constants;
mod cli;

fn main() {
    let mut cli = cli::Cli::new().unwrap();

    cli.run().unwrap();
}