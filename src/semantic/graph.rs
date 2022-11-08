use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use crate::token::{Token, TokenType};

use super::utils::append_id;

#[derive(Debug, Clone, Hash)]
pub struct Node {
    pub op: TokenType,
    pub lexeme: String,
    pub is_leaf: bool,
    pub left: u64,
    pub right: u64,
}

impl Node {
    pub fn from_num(token: &Token) -> Self {
        Node {
            op: token.token_type.clone(),
            lexeme: token.lexeme.clone(),
            is_leaf: true,
            left: 0,
            right: 0,
        }
    }

    pub fn from_var(token: &Token, hash: u64) -> Self {
        Node {
            op: token.token_type.clone(),
            lexeme: append_id(&token.lexeme),
            is_leaf: true,
            left: hash,
            right: 0,
        }
    }

    pub fn from_op(token: &Token, left: u64, right: u64) -> Self {
        Node {
            op: token.token_type.clone(),
            lexeme: token.lexeme.clone(),
            is_leaf: false,
            left,
            right,
        }
    }

    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Clone)]
pub struct NodeWithIndex {
    pub node: Node,
    pub index: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub table: HashMap<u64, NodeWithIndex>,
    pub stack: Vec<u64>,
}

impl NodeWithIndex {
    pub fn new(node: Node, index: usize) -> Self {
        NodeWithIndex { index, node }
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            table: HashMap::new(),
            stack: Vec::new(),
        }
    }
    pub fn add(&mut self, node: Node) -> u64 {
        let hash = node.get_hash();
        if let None = self.table.get(&hash) {
            let node_with_index = NodeWithIndex::new(node.clone(), self.stack.len());
            self.table.insert(hash, node_with_index);
            self.stack.push(hash);
        }
        hash
    }

    pub fn get(&self, hash: &u64) -> Option<&NodeWithIndex> {
        self.table.get(hash)
    }
}
