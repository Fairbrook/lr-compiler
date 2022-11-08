use core::fmt;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum ProductionItem {
    Leaf(Token),
    Production(Production),
}

#[derive(Debug, Clone)]
pub enum ProductionType {
    Programa,
    Declaraciones,
    SigDeclaraciones,
    Declaracion,
    ListaVariables,
    SigListaVariables,
    Ordenes,
    SigOrdenes,
    Orden,
    Condicion,
    SigCondicion,
    Comparacion,
    Operador,
    Numeros,
    BucleWhile,
    Asignar,
    ExpresionArit,
    RestExp,
    Term,
    RestTerm,
    Factor,
}

#[derive(Debug, Clone)]
pub struct Production {
    pub production_type: ProductionType,
    pub items: Vec<ProductionItem>,
}

impl Production {
    pub fn new(production_type: ProductionType) -> Self {
        Production {
            production_type,
            items: Vec::new(),
        }
    }

    pub fn push_node(&mut self, production: Production) {
        self.items.push(ProductionItem::Production(production));
    }

    pub fn push_leaf(&mut self, leaf: Token) {
        self.items.push(ProductionItem::Leaf(leaf));
    }

    pub fn to_string(&self, prepend: &str) -> String {
        let strings: Vec<String> = self
            .items
            .iter()
            .map(|item| match item {
                ProductionItem::Production(prod) => prod.to_string(&format!("{}{}", prepend, "│")),
                ProductionItem::Leaf(leaf) => format!("{}│├ {}", prepend, leaf.lexeme),
            })
            .collect();
        let joined = strings.join("\n");
        format!(
            "{}├ {}{}{}",
            prepend,
            production_type_to_str(&self.production_type),
            if self.items.len() > 0 { "\n" } else { "" },
            joined,
        )
        .to_string()
    }
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string(""))
    }
}

pub fn production_type_to_str(production_type: &ProductionType) -> String {
    match production_type {
        ProductionType::Programa => "programa",
        ProductionType::Declaraciones => "declaraciones",
        ProductionType::SigDeclaraciones => "sig_declaraciones",
        ProductionType::Declaracion => "declaración",
        ProductionType::ListaVariables => "lista_variables",
        ProductionType::SigListaVariables => "sig_lista_variables",
        ProductionType::Ordenes => "ordenes",
        ProductionType::SigOrdenes => "sig_ordenes",
        ProductionType::Orden => "orden",
        ProductionType::Condicion => "condicion",
        ProductionType::SigCondicion => "sig_condicion",
        ProductionType::Comparacion => "comparación",
        ProductionType::Operador => "operador",
        ProductionType::Numeros => "numeros",
        ProductionType::BucleWhile => "bucle_while",
        ProductionType::Asignar => "asignar",
        ProductionType::ExpresionArit => "expresion_arit",
        ProductionType::RestExp => "rest_expr",
        ProductionType::Term => "termino",
        ProductionType::RestTerm => "rest_term",
        ProductionType::Factor => "factor",
    }
    .to_string()
}
