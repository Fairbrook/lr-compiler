use crate::{production::Production, symbols::SymbolsTable};

use super::{
    error::SemanticError,
    graph::{Graph, Node},
    utils::{production_as_leaf, production_as_node},
};

pub struct ExpressionAnalyzer {
    symbols_table: SymbolsTable,
    pub graph: Graph,
}

pub type ExpressionResult = Result<Graph, SemanticError>;
pub type IntermediateResult = Result<u64, SemanticError>;

impl ExpressionAnalyzer {
    pub fn from(table: &SymbolsTable) -> Self {
        ExpressionAnalyzer {
            symbols_table: table.clone(),
            graph: Graph::new(),
        }
    }

    pub fn numeros(&mut self, prod: &Production) -> IntermediateResult {
        let token = production_as_leaf(&prod.items[0])?;
        Ok(self.graph.add(Node::from_num(&token)))
    }

    pub fn operador(&mut self, prod: &Production) -> IntermediateResult {
        if let Ok(token) = production_as_leaf(&prod.items[0]) {
            return match self.symbols_table.get_hash_if_set(&token) {
                Some(hash) => Ok(self.graph.add(Node::from_var(&token, hash))),
                None => Err(SemanticError::from_undefined(token.clone())),
            };
        }
        self.numeros(production_as_node(&prod.items[0])?)
    }

    pub fn factor(&mut self, prod: &Production) -> IntermediateResult {
        if let Ok(_) = production_as_leaf(&prod.items[0]) {
            return self.expresion_arit(production_as_node(&prod.items[1])?);
        }
        return self.operador(production_as_node(&prod.items[0])?);
    }

    pub fn rest_term(&mut self, prod: &Production, previous: u64) -> IntermediateResult {
        if prod.items.is_empty() {
            return Ok(previous);
        }
        let op = production_as_leaf(&prod.items[0])?;
        let factor = self.factor(production_as_node(&prod.items[1])?)?;
        let rest = self.rest_term(production_as_node(&prod.items[2])?, factor)?;
        Ok(self.graph.add(Node::from_op(op, previous, rest)))
    }

    pub fn termino(&mut self, prod: &Production) -> IntermediateResult {
        let factor = self.factor(production_as_node(&prod.items[0])?)?;
        self.rest_term(production_as_node(&prod.items[1])?, factor)
    }

    pub fn rest_expr(&mut self, prod: &Production, previous: u64) -> IntermediateResult {
        if prod.items.is_empty() {
            return Ok(previous);
        }
        let op = production_as_leaf(&prod.items[0])?;
        let factor = self.termino(production_as_node(&prod.items[1])?)?;
        let rest = self.rest_expr(production_as_node(&prod.items[2])?, factor)?;
        Ok(self.graph.add(Node::from_op(op, previous, rest)))
    }

    pub fn expresion_arit(&mut self, prod: &Production) -> IntermediateResult {
        let term = self.termino(production_as_node(&prod.items[0])?)?;
        self.rest_expr(production_as_node(&prod.items[1])?, term)
    }
}
