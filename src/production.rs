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
    ExpArit,
}

#[derive(Debug, Clone)]
pub struct Production {
    pub production_type: ProductionType,
    pub items: Vec<ProductionItem>,
}

pub fn production_type_to_str(production_type: ProductionType) -> String {
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
        ProductionType::ExpArit => "exp_arit",
    }
    .to_string()
}
