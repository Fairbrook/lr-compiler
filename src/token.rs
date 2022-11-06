#[derive(Debug, Clone, PartialEq)]
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
    OperadorArit,
    OperadorCondicion,
    OperadorAsig,
    While,
    Endwhile,
    EOF,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
    pub col: u32,
}

pub fn token_type_to_str(token_type: TokenType) -> String {
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
        TokenType::OperadorArit => "operador",
        TokenType::OperadorCondicion => "operador condicional",
        TokenType::OperadorAsig => ":=",
        TokenType::While => "while",
        TokenType::Endwhile => "endwhile",
        TokenType::EOF => "EOF",
        TokenType::Unknown => "No reconocido",
    }
    .to_string()
}
