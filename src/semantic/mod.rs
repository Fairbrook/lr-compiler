pub mod error;
pub mod expresion;
pub mod graph;
pub mod utils;

use crate::{
    production::{Production, ProductionItem, ProductionType},
    sintactic::SintacticAnalyzer,
    symbols::{SymbolsTable, VariableType},
    token::TokenType,
};

use self::{
    error::SemanticError,
    expresion::ExpressionAnalyzer,
    utils::{append_id, production_as_leaf, production_as_node},
};

pub struct SemanticAnalyzer {
    pub table: SymbolsTable,
    current_jump: u32,
    current_temp: u32,
}

pub type SemanticRepresentation = String;
pub type SemanticResult = Result<SemanticRepresentation, SemanticError>;

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            table: SymbolsTable::new(),
            current_jump: 0,
            current_temp: 0,
        }
    }

    pub fn next_jump(&mut self) -> String {
        self.current_jump += 1;
        let tag = format!("jmp_{}", self.current_jump);
        tag
    }

    pub fn next_temp(&mut self) -> String {
        self.current_temp += 1;
        let tag = format!("__temp_{}", self.current_temp);
        tag
    }

    pub fn current_temp(&mut self) -> String {
        let tag = format!("__temp_{}", self.current_temp);
        tag
    }

    pub fn parse(&mut self, input: &str) -> SemanticResult {
        self.table.clear();
        self.current_temp = 0;
        self.current_jump = 0;

        let input_cloned = input.to_string();
        let mut sintactic_analyzer = SintacticAnalyzer::new(input_cloned.as_str());

        let tree = match sintactic_analyzer.analize() {
            Err(sintactic_error) => Err(SemanticError::from_sintactic(sintactic_error)),
            Ok(sintactic_tree) => Ok(sintactic_tree),
        }?;
        self.declaraciones(production_as_node(&tree.items[1])?)?;
        self.ordenes(production_as_node(&tree.items[2])?)
    }

    pub fn sig_lista_variables(
        &mut self,
        var_type: &VariableType,
        lista: &Production,
    ) -> Result<(), SemanticError> {
        if lista.items.len() > 0 {
            return self.lista_variables(var_type, production_as_node(&lista.items[1])?);
        }
        Ok(())
    }

    pub fn lista_variables(
        &mut self,
        var_type: &VariableType,
        lista: &Production,
    ) -> Result<(), SemanticError> {
        let id = production_as_leaf(&lista.items[0])?;
        let next_list = production_as_node(&lista.items[1])?;
        self.table.add(&id, var_type);
        self.sig_lista_variables(var_type, next_list)
    }

    pub fn declaracion(&mut self, production: &Production) -> Result<(), SemanticError> {
        let tipo = production_as_leaf(&production.items[0])?;
        let lista = production_as_node(&production.items[1])?;
        let var_type = match tipo.lexeme.as_str() {
            "entero" => VariableType::Entero,
            "real" => VariableType::Real,
            _ => VariableType::Real,
        };
        self.lista_variables(&var_type, &lista)
    }

    pub fn declaraciones(&mut self, production: &Production) -> Result<(), SemanticError> {
        if production.items.len() == 3 {
            let declaracion = production_as_node(&production.items[0])?;
            let sig_declaraciones = production_as_node(&production.items[2])?;
            self.declaracion(&declaracion)?;
            self.declaraciones(&sig_declaraciones)?;
        }
        Ok(())
    }

    pub fn operador(&mut self, production: &Production) -> SemanticResult {
        let operador = &production.items[0];
        match operador {
            ProductionItem::Leaf(op) => Ok(append_id(&op.lexeme)),
            ProductionItem::Production(num) => {
                if let ProductionItem::Leaf(num) = &num.items[0] {
                    Ok(num.lexeme.clone())
                } else {
                    Ok(String::new())
                }
            }
        }
    }

    pub fn comparacion(&mut self, production: &Production) -> SemanticResult {
        let operador_a = production_as_node(&production.items[0])?;
        let op = production_as_leaf(&production.items[1])?;
        let operador_b = production_as_node(&production.items[2])?;
        Ok(format!(
            "{} {} {}",
            self.operador(operador_a)?,
            op.lexeme,
            self.operador(operador_b)?
        ))
    }

    pub fn exp(&mut self, production: &Production) -> SemanticResult {
        let mut analyzer = ExpressionAnalyzer::from(&self.table);
        analyzer.expresion_arit(production)?;
        let graph = analyzer.graph;
        let tags: Vec<String> = graph.stack.iter().map(|_| self.next_temp()).collect();
        let instructions = graph
            .stack
            .iter()
            .map(|hash| match graph.get(hash) {
                Some(node_with_index) => match node_with_index.node.is_leaf {
                    false => {
                        let left = node_with_index.node.left;
                        let right = node_with_index.node.right;
                        if let (Some(left), Some(right)) = (graph.get(&left), graph.get(&right)) {
                            let left_tag = &tags[left.index];
                            let right_tag = &tags[right.index];
                            return format!(
                                "{} := {} {} {}\n",
                                tags[node_with_index.index],
                                left_tag,
                                node_with_index.node.lexeme,
                                right_tag
                            );
                        }
                        String::new()
                    }
                    true => format!(
                        "{} := {}\n",
                        tags[node_with_index.index], node_with_index.node.lexeme
                    ),
                },
                None => String::new(),
            })
            .collect::<Vec<String>>()
            .join("");
        Ok(instructions)
    }

    pub fn asignar(&mut self, production: &Production) -> SemanticResult {
        let id = &production.items[0];
        let exp = &production.items[2];
        let mut res = String::new();
        if let (ProductionItem::Leaf(id), ProductionItem::Production(exp)) = (id, exp) {
            res.push_str(&self.exp(&exp)?);
            res.push_str(&format!(
                "{} := {}",
                append_id(&id.lexeme),
                self.current_temp()
            ));
        }
        Ok(res)
    }

    pub fn bucle_while(&mut self, production: &Production) -> SemanticResult {
        let condicion = &production.items[2];
        let ordenes = &production.items[4];
        let mut res = String::new();
        if let (ProductionItem::Production(condicion), ProductionItem::Production(ordenes)) =
            (condicion, ordenes)
        {
            let start_tag = self.next_jump();
            let end_tag = self.next_jump();
            res.push_str(&format!("{}:\n", start_tag));
            res.push_str(&format!("if false "));
            res.push_str(&self.comparacion(&condicion)?);
            res.push_str(&format!(" jump to {}\n", end_tag));
            res.push_str(&self.ordenes(&ordenes)?);
            res.push_str(&format!("jump to {} \n", start_tag));
            res.push_str(&format!("{}:\n", end_tag));
        }
        Ok(res)
    }

    pub fn sig_condicion(&mut self, label: &str, production: &Production) -> SemanticResult {
        if let ProductionItem::Leaf(token) = &production.items[0] {
            return match token.token_type {
                TokenType::End => Ok(format!("{}:\n", label)),
                TokenType::Else => {
                    let mut res = String::new();
                    let jump = self.next_jump();
                    res.push_str(&format!("jump to {}\n", jump));
                    res.push_str(&format!("{}:\n", label));
                    if let ProductionItem::Production(sig) = &production.items[1] {
                        res.push_str(&self.ordenes(&sig)?);
                    }
                    res.push_str(&format!("{}:\n", jump));
                    Ok(res)
                }
                _ => Ok(String::new()),
            };
        }
        Ok(String::new())
    }
    pub fn condicion(&mut self, production: &Production) -> SemanticResult {
        let condicion = production_as_node(&production.items[2])?;
        let ordenes = production_as_node(&production.items[4])?;
        let sig_condicion = production_as_node(&production.items[5])?;
        let jump = self.next_jump();
        Ok(format!(
            "if false {} jump to {}\n{}{}",
            self.comparacion(condicion)?,
            jump,
            self.ordenes(ordenes)?,
            self.sig_condicion(&jump, &sig_condicion)?,
        ))
    }

    pub fn orden(&mut self, production: &Production) -> SemanticResult {
        let orden = production_as_node(&production.items[0])?;
        let ordenes = match orden.production_type {
            ProductionType::Condicion => self.condicion(orden),
            ProductionType::BucleWhile => self.bucle_while(orden),
            ProductionType::Asignar => self.asignar(orden),
            _ => Ok(String::new()),
        }?;
        return Ok(format!("{}\n", ordenes));
    }

    pub fn ordenes(&mut self, production: &Production) -> SemanticResult {
        let mut parsed = String::new();
        if production.items.len() == 3 {
            let orden = production_as_node(&production.items[0])?;
            let sig_ordenes = production_as_node(&production.items[2])?;
            parsed.push_str(self.orden(&orden)?.as_str());
            parsed.push_str(self.ordenes(&sig_ordenes)?.as_str());
        }
        Ok(parsed)
    }
}
