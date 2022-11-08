use std::{env::args, fs};

use semantic::SemanticAnalyzer;

pub mod lexic;
pub mod production;
pub mod semantic;
pub mod sintactic;
pub mod symbols;
pub mod token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = args().nth(1).expect("No file path given");
    let contets = fs::read_to_string(path)?;
    let mut semantic = SemanticAnalyzer::new();
    let res = semantic.parse(&contets)?;
    println!("{}", res);
    Ok(())
}
