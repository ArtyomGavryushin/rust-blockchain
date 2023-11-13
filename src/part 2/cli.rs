use crate::blockchain::Blockchain;
use crate::errors::Result;
use clap::Command;
use clap::arg;

pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli>{
        Ok (Cli {
            bc: Blockchain::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("Artyom")
            .about("Blockchain in rust - simple realization")
            .subcommand(Command::new("printchain").about("print all the chain blocks"))
            .subcommand(
                Command::new("addblock")
                    .about("add a block in the blockchain")
                    .arg(arg!(<DATA>" 'the blockchain data'")),
            )
            .get_matches();
        if let Some(ref matches) = matches.subcommand_matches("addblock"){
            if let Some(c) = matches.get_one::<String>("DATA"){
                self.addblock(String::from(c))?;
            }else {
                println!("Not printing testing lists...");
            }
        }

        if let Some(_) = matches.subcommand_matches("printchain"){
            self.print_chain();
        }

        Ok(())
    }

    fn addblock(&mut self, data: String) -> Result<()> {
        self.bc.add_block(data)
    }

    fn print_chain(&mut self){
        for b in &mut self.bc.iter() {
            println!("block: {:#?}", b);
        }
    }
}