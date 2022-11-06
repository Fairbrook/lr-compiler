use std::{error, fmt};

use crate::token::Token;

#[derive(Debug, Clone, Default)]
pub struct SintacticError {
    token: Token,
    expected: String,
}
impl SintacticError {
    pub fn new(token: &Token, expected: &str) -> Self {
        SintacticError {
            expected: String::from(expected),
            token: token.clone(),
        }
    }
}
impl error::Error for SintacticError {}
impl fmt::Display for SintacticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Caracter inesperado '{}', en la linea {} columna {}, se esperaba: '{}'",
            self.token.lexeme, self.token.line, self.token.col, self.expected
        )
    }
}
