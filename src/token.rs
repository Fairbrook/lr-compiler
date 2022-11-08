use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Default, Hash)]
pub enum TokenType {
    Tipo,
    Begin,
    End,
    Id,
    Entero,
    Real,
    Coma,
    Punto,
    Semicolon,
    If,
    ParentesisAbierto,
    ParentesisCerrado,
    Else,
    OperadorAritA,
    OperadorAritB,
    OperadorCondicion,
    OperadorAsig,
    While,
    Endwhile,
    EOF,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Default)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
    pub col: u32,
}

impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.lexeme.hash(state);
        self.token_type.hash(state);
    }
}

pub fn token_type_to_str(token_type: &TokenType) -> String {
    match token_type {
        TokenType::Tipo => "tipo",
        TokenType::Begin => "begin",
        TokenType::End => "end",
        TokenType::Id => "Id",
        TokenType::Entero => "entero",
        TokenType::Real => "real",
        TokenType::Coma => ",",
        TokenType::Punto => ".",
        TokenType::Semicolon => ";",
        TokenType::If => "if",
        TokenType::ParentesisAbierto => ")",
        TokenType::ParentesisCerrado => ")",
        TokenType::Else => "else",
        TokenType::OperadorAritA => "+ o -",
        TokenType::OperadorAritB => "/ o *",
        TokenType::OperadorCondicion => "operador condicional",
        TokenType::OperadorAsig => ":=",
        TokenType::While => "while",
        TokenType::Endwhile => "endwhile",
        TokenType::EOF => "EOF",
        TokenType::Unknown => "No reconocido",
    }
    .to_string()
}
