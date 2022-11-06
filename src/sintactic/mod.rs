pub mod error;

use crate::{
    lexic::LexicAnalyzer,
    production::{Production, ProductionItem, ProductionType},
    token::{token_type_to_str, Token, TokenType},
};

use self::error::SintacticError;

pub type ItermediateRep = Production;
pub type SintacticResult = Result<ItermediateRep, SintacticError>;

pub struct SintacticAnalyzer<'a> {
    pub lexic: LexicAnalyzer<'a>,
    last_token: Token,
}

impl<'a> SintacticAnalyzer<'a> {
    pub fn new(input: &'a str) -> Self {
        SintacticAnalyzer {
            lexic: LexicAnalyzer::new(input),
            last_token: Token::default(),
        }
    }

    pub fn next_token(&mut self) {
        self.last_token = self.lexic.next_token();
    }

    pub fn is_last(&self, token_type: TokenType) -> Result<(), SintacticError> {
        if token_type != self.last_token.token_type {
            return Err(SintacticError::new(
                &self.last_token,
                &token_type_to_str(&token_type),
            ));
        } else {
            return Ok(());
        }
    }

    pub fn sig_lista_variables(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::SigListaVariables);
        while let TokenType::Coma = self.last_token.token_type {
            prod.items
                .push(ProductionItem::Leaf(self.last_token.clone()));
            self.next_token();
            let rest_var = self.lista_variables()?;
            prod.items.push(ProductionItem::Production(rest_var));
        }
        Ok(prod)
    }

    pub fn lista_variables(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::ListaVariables);
        self.is_last(TokenType::Id)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();
        let sig = self.sig_lista_variables()?;
        prod.items.push(ProductionItem::Production(sig));
        Ok(prod)
    }

    pub fn declaracion(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Declaracion);
        self.is_last(TokenType::Tipo)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();
        let lista = self.lista_variables()?;
        prod.items.push(ProductionItem::Production(lista));
        Ok(prod)
    }

    pub fn sig_declaraciones(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::SigDeclaraciones);
        if let TokenType::Tipo = self.last_token.token_type {
            let declaraciones = self.declaracion()?;
            prod.items.push(ProductionItem::Production(declaraciones));

            self.is_last(TokenType::Semicolon)?;
            prod.items
                .push(ProductionItem::Leaf(self.last_token.clone()));
            self.next_token();

            let sig = self.sig_declaraciones()?;
            prod.items.push(ProductionItem::Production(sig));
        }
        Ok(prod)
    }

    pub fn declaraciones(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Declaraciones);
        let declaracion = self.declaracion()?;
        prod.items.push(ProductionItem::Production(declaracion));

        self.is_last(TokenType::Semicolon)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();

        let sig = self.sig_declaraciones()?;
        prod.items.push(ProductionItem::Production(sig));
        Ok(prod)
    }

    pub fn sig_ordenes(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::SigOrdenes);
        if self.is_orden() {
            let orden = self.orden()?;
            prod.items.push(ProductionItem::Production(orden));
            self.is_last(TokenType::Semicolon)?;
            prod.items
                .push(ProductionItem::Leaf(self.last_token.clone()));
            self.next_token();

            let sig = self.sig_ordenes()?;
            prod.items.push(ProductionItem::Production(sig));
        }
        Ok(prod)
    }

    pub fn sig_condicion(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::SigCondicion);
        if let TokenType::Else = self.last_token.token_type {
            prod.items
                .push(ProductionItem::Leaf(self.last_token.clone()));
            self.next_token();

            let ordenes = self.ordenes()?;
            prod.items.push(ProductionItem::Production(ordenes));
        }
        self.is_last(TokenType::End)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();
        return Ok(prod);
    }

    pub fn numeros(&mut self) -> SintacticResult {
        if let TokenType::Entero | TokenType::Real = self.last_token.token_type {
            let mut prod = Production::new(ProductionType::Numeros);
            prod.items
                .push(ProductionItem::Leaf(self.last_token.clone()));
            self.next_token();
            return Ok(prod);
        }
        Err(SintacticError::new(
            &self.last_token,
            "número entero o real",
        ))
    }

    pub fn operador(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Operador);
        if let TokenType::Id = self.last_token.token_type {
            prod.items
                .push(ProductionItem::Leaf(self.last_token.clone()));
            self.next_token();
            return Ok(prod);
        }
        let num = self.numeros()?;
        prod.items.push(ProductionItem::Production(num));
        Ok(prod)
    }

    pub fn comparacion(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Comparacion);
        let operador = self.operador()?;
        prod.items.push(ProductionItem::Production(operador));
        self.is_last(TokenType::OperadorCondicion)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();
        let operador = self.operador()?;
        prod.items.push(ProductionItem::Production(operador));
        Ok(prod)
    }

    pub fn condicion(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Condicion);
        self.is_last(TokenType::If)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();

        self.is_last(TokenType::ParentesisAbierto)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();

        let comparacion = self.comparacion()?;
        prod.items.push(ProductionItem::Production(comparacion));

        self.is_last(TokenType::ParentesisCerrado)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();

        let ordenes = self.ordenes()?;
        prod.items.push(ProductionItem::Production(ordenes));

        let sig = self.sig_condicion()?;
        prod.items.push(ProductionItem::Production(sig));
        Ok(prod)
    }

    pub fn bucle_while(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::BucleWhile);

        self.is_last(TokenType::While)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();

        self.is_last(TokenType::ParentesisAbierto)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();

        let comparacion = self.comparacion()?;
        prod.items.push(ProductionItem::Production(comparacion));

        self.is_last(TokenType::ParentesisCerrado)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();

        let ordenes = self.ordenes()?;
        prod.items.push(ProductionItem::Production(ordenes));

        self.is_last(TokenType::Endwhile)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();
        Ok(prod)
    }

    pub fn exp_arit(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::ExpArit);
        if let TokenType::OperadorArit = self.last_token.token_type {
            prod.items
                .push(ProductionItem::Leaf(self.last_token.clone()));
            self.next_token();

            let exp = self.expresion_arit()?;
            prod.items.push(ProductionItem::Production(exp));

            let exp = self.exp_arit()?;
            prod.items.push(ProductionItem::Production(exp));
        }
        Ok(prod)
    }

    pub fn expresion_arit(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::ExpresionArit);
        match self.last_token.token_type {
            TokenType::ParentesisAbierto => {
                prod.items
                    .push(ProductionItem::Leaf(self.last_token.clone()));
                self.next_token();

                let exp = self.expresion_arit()?;
                prod.items.push(ProductionItem::Production(exp));

                let op = self.operador()?;
                prod.items.push(ProductionItem::Production(op));

                let exp = self.expresion_arit()?;
                prod.items.push(ProductionItem::Production(exp));

                self.is_last(TokenType::ParentesisCerrado)?;
                prod.items
                    .push(ProductionItem::Leaf(self.last_token.clone()));
                self.next_token();
            }
            TokenType::Id => {
                prod.items
                    .push(ProductionItem::Leaf(self.last_token.clone()));
                self.next_token();
            }
            TokenType::Entero | TokenType::Real => {
                let numeros = self.numeros()?;
                prod.items.push(ProductionItem::Production(numeros));
            }
            _ => {
                return Err(SintacticError::new(
                    &self.last_token,
                    "parentesis, identificador o un número",
                ));
            }
        }

        let exp = self.exp_arit()?;
        prod.items.push(ProductionItem::Production(exp));
        Ok(prod)
    }

    pub fn asignar(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Asignar);
        self.is_last(TokenType::Id)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();

        self.is_last(TokenType::OperadorAsig)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();

        let exp = self.expresion_arit()?;
        prod.items.push(ProductionItem::Production(exp));
        Ok(prod)
    }

    pub fn is_orden(&mut self) -> bool {
        match self.last_token.token_type {
            TokenType::If => true,
            TokenType::While => true,
            TokenType::Id => true,
            _ => false,
        }
    }

    pub fn orden(&mut self) -> SintacticResult {
        let content = match self.last_token.token_type {
            TokenType::If => self.condicion(),
            TokenType::While => self.bucle_while(),
            TokenType::Id => self.asignar(),
            _ => Err(SintacticError::new(
                &self.last_token,
                "if while o una asignación",
            )),
        };
        if let Ok(production) = content {
            let mut prod = Production::new(ProductionType::Orden);
            prod.items.push(ProductionItem::Production(production));
            return Ok(prod);
        }
        content
    }

    pub fn ordenes(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Ordenes);
        let orden = self.orden()?;
        prod.items.push(ProductionItem::Production(orden));

        self.is_last(TokenType::Semicolon)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();

        let sig = self.sig_ordenes()?;
        prod.items.push(ProductionItem::Production(sig));
        Ok(prod)
    }

    pub fn programa(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Programa);
        self.is_last(TokenType::Begin)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();

        let declaraciones = self.declaraciones()?;
        prod.items.push(ProductionItem::Production(declaraciones));

        let ordenes = self.ordenes()?;
        prod.items.push(ProductionItem::Production(ordenes));

        self.is_last(TokenType::End)?;
        prod.items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();
        Ok(prod)
    }

    pub fn analize(&'a mut self) -> SintacticResult {
        self.next_token();
        let production = self.programa()?;
        self.is_last(TokenType::EOF)?;
        Ok(production)
    }
}
