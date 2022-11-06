use lexic::LexicAnalyzer;
use token::TokenType;

pub mod lexic;
pub mod production;
pub mod token;

fn main() {
    let mut lexic = LexicAnalyzer::new("hola hola12 123 123.32 begin end entero real , ; if () else = <= >= <> < > := \n$ while endwhile + - * / - + = * /");
    let mut token = lexic.next_token();
    while token.token_type != TokenType::EOF {
        println!("{:?}", token);
        token = lexic.next_token();
    }
}
