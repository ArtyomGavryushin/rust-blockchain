use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::transaction::Transaction;
use crate::wallet::Wallets;
use clap::Command;
use clap::arg;
use std::process::exit;

pub struct Cli {

}

impl Cli {
    pub fn new() -> Result<Cli>{
        Ok (Cli {  })
    }

    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("Artyom")
            .about("Blockchain in rust - simple realization")

            .subcommand(Command::new("printchain")
                .about("print all the chain blocks"))

            .subcommand(Command::new("createwallet")
                .about("create a wallet"))

            .subcommand(Command::new("listaddresses")
                .about("list all addresses"))

            .subcommand(Command::new("getbalance")
                .about("get balance in the blockchain")
                .arg(arg!(<ADDRESS>"'The Address it get balance for'")
            ))

            .subcommand(Command::new("create")
                .about("Creating new blockchain")
                .arg(arg!(<ADDRESS>"'The address to send genesis block reward to' ")
            ))

            .subcommand(
                Command::new("send")
                    .about("send in the blockchain")
                    .arg(arg!(<FROM>" 'Source wallet address'"))
                    .arg(arg!(<TO>" 'Destination wallet address'"))
                    .arg(arg!(<AMOUNT>" 'Destination wallet address")),
            )

            .get_matches();

        if let Some(_) = matches.subcommand_matches("createwallet") {
            let mut ws = Wallets::new()?;
            let address = ws.create_wallet();
            ws.save_all()?;
            println!("Success: address {}", address);
        }

        if let Some(_) = matches.subcommand_matches("listaddresses"){
            let ws = Wallets::new()?;
            let addresses = ws.get_all_address();
            println!("addresses: ");
            for ad in addresses{
                println!("{}", ad);
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("create"){
            if let Some(address) = matches.get_one::<String>("ADDRESS"){
                let address = String::from(address);
                Blockchain::create_blockchain(address.clone())?;
                println!("create blockchain");
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("getbalance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                let bc = Blockchain::new()?;
                let utxos = bc.find_UTXO(&address);
                let mut blance = 0;
                for out in utxos{
                    blance += out.value;
                }

                println!("Balance of '{}'; {} ", address, blance);
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("send"){
            let from = if let Some(address) = matches.get_one::<String>("FROM"){
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let to = if let Some(address) = matches.get_one::<String>("TO") {
                address
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let amount: i32 = if let Some(amount) = matches.get_one::<String>("AMOUNT"){
                amount.parse()?
            } else {
                println!("from not supply!: usage");
                exit(1)
            };

            let mut bc = Blockchain::new()?;
            let tx = Transaction::new_UTXO(from, to, amount, &bc)?;
            bc.add_block(vec![tx])?;
            println!("Success!");

        }


        if let Some(_) = matches.subcommand_matches("printchain"){
            let bc = Blockchain::new()?;
            for b in &mut bc.iter(){
                println!("block: {:#?}", b);
            }
        }

        Ok(())
    }


}