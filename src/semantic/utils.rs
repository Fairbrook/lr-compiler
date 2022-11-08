use crate::{
    production::{Production, ProductionItem},
    token::Token,
};

use super::error::SemanticError;

pub fn production_as_node(item: &ProductionItem) -> Result<&Production, SemanticError> {
    match item {
        ProductionItem::Production(prod) => Ok(prod),
        ProductionItem::Leaf(token) => Err(SemanticError::from_unexpected(token.clone())),
    }
}

pub fn production_as_leaf(item: &ProductionItem) -> Result<&Token, SemanticError> {
    match item {
        ProductionItem::Production(prod) => Err(SemanticError::from_format(prod.clone())),
        ProductionItem::Leaf(token) => Ok(token),
    }
}

pub fn append_id(id: &str) -> String {
    format!("_{}", id)
}
