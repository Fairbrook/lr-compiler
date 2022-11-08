use std::{error, fmt};

use crate::{
    production::{production_type_to_str, Production},
    sintactic::error::SintacticError,
    token::Token,
};

#[derive(Debug, Clone)]
pub enum SemanticErrorType {
    Sintactic(SintacticError),
    Undefined(Token),
    BadFormat(Production),
    Unexpected(Token),
}

#[derive(Debug, Clone)]
pub struct SemanticError {
    error_type: SemanticErrorType,
}

impl SemanticError {
    pub fn from_sintactic(error: SintacticError) -> Self {
        SemanticError {
            error_type: SemanticErrorType::Sintactic(error),
        }
    }

    pub fn from_undefined(token: Token) -> Self {
        SemanticError {
            error_type: SemanticErrorType::Undefined(token),
        }
    }

    pub fn from_format(production: Production) -> Self {
        SemanticError {
            error_type: SemanticErrorType::BadFormat(production),
        }
    }
    pub fn from_unexpected(token: Token) -> Self {
        SemanticError {
            error_type: SemanticErrorType::Unexpected(token),
        }
    }
}
impl error::Error for SemanticError {}
impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.error_type {
            SemanticErrorType::BadFormat(prod) => write!(
                f,
                "Producción con formato inesperado {}",
                production_type_to_str(&prod.production_type)
            ),
            SemanticErrorType::Undefined(token) => writeln!(
                f,
                "Utilizacion de una variable no declarada '{}' en la linea {} columna {}",
                token.lexeme, token.line, token.col
            ),
            SemanticErrorType::Unexpected(token) => writeln!(
                f,
                "Caracter inesperado '{}' en la linea {} columna {}",
                token.lexeme, token.line, token.col
            ),
            SemanticErrorType::Sintactic(sintactic) => sintactic.fmt(f),
        }
    }
}
