use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum VariableType {
    Entero,
    Real,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub token: Token,
    pub index: usize,
    pub variable_type: VariableType,
}

impl Variable {
    pub fn new(token: Token, index: usize, variable_type: VariableType) -> Self {
        Variable {
            token,
            index,
            variable_type,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SymbolsTable {
    pub table: HashMap<u64, Variable>,
    pub stack: Vec<u64>,
}

impl SymbolsTable {
    pub fn new() -> Self {
        SymbolsTable {
            table: HashMap::new(),
            stack: Vec::new(),
        }
    }
    pub fn add(&mut self, token: &Token, variable_type: &VariableType) -> u64 {
        let mut hasher = DefaultHasher::new();
        token.hash(&mut hasher);
        let hash = hasher.finish();
        if let None = self.table.get(&hash) {
            let token_with_index =
                Variable::new(token.clone(), self.stack.len(), variable_type.clone());
            self.table.insert(hash, token_with_index);
            self.stack.push(hash);
        }
        hash
    }

    pub fn clear(&mut self) {
        self.table.clear();
        self.stack.clear();
    }

    pub fn get(&self, hash: &u64) -> Option<&Variable> {
        self.table.get(hash)
    }

    pub fn get_from_token(&self, token: &Token) -> Option<&Variable> {
        let mut hasher = DefaultHasher::new();
        token.hash(&mut hasher);
        let hash = hasher.finish();
        self.table.get(&hash)
    }

    pub fn get_hash_if_set(&self, token: &Token) -> Option<u64> {
        match self.get_from_token(token) {
            Some(variable) => Some(self.stack[variable.index]),
            None => None,
        }
    }
}
