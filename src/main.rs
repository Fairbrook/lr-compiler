use std::{env::args, fs};

use sintactic::SintacticAnalyzer;

pub mod lexic;
pub mod production;
pub mod sintactic;
pub mod token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = args().nth(1).expect("No file path given");
    let contets = fs::read_to_string(path)?;
    let mut lexic = SintacticAnalyzer::new(&contets);
    let res = lexic.analize();
    if let Ok(r) = &res {
        println!("{}", r);
    };
    if let Err(e) = res {
        println!("{}", e);
    };
    Ok(())
}
