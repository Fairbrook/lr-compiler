use sintactic::SintacticAnalyzer;

pub mod lexic;
pub mod production;
pub mod sintactic;
pub mod token;

fn main() {
    let mut lexic = SintacticAnalyzer::new(
        "begin real hola; hola := 10; if(hola > 10) hola :=5; else hola:=6; end; end",
    );
    let res = lexic.analize();
    if let Ok(r) = &res {
        println!("{}", r);
    }
    if let Err(e) = res{
        println!("{}", e);
    }
}
