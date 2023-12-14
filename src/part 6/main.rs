use crate::cli::Cli;
use crate::errors::Result;

mod block;
mod transaction;
mod errors;
mod blockchain;
mod cli;
mod tx;
mod wallet;



fn main () -> Result<()>{
    let mut cli = Cli::new();
    cli?.run()?;

    Ok(())
}