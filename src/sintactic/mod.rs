pub mod error;

use crate::{
    lexic::LexicAnalyzer,
    production::{Production, ProductionType},
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

    pub fn is_last(&self, token_type: &TokenType) -> Result<(), SintacticError> {
        if *token_type != self.last_token.token_type {
            return Err(SintacticError::new(
                &self.last_token,
                &token_type_to_str(&token_type),
            ));
        } else {
            return Ok(());
        }
    }

    pub fn push_token_if(
        &mut self,
        token_type: &TokenType,
        production: &mut Production,
    ) -> Result<(), SintacticError> {
        self.is_last(token_type)?;
        production.push_leaf(self.last_token.clone());
        self.next_token();
        Ok(())
    }

    pub fn sig_lista_variables(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::SigListaVariables);
        while let Ok(_) = self.push_token_if(&TokenType::Coma, &mut prod) {
            prod.push_node(self.lista_variables()?);
        }
        Ok(prod)
    }

    pub fn lista_variables(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::ListaVariables);
        self.push_token_if(&TokenType::Id, &mut prod)?;
        prod.push_node(self.sig_lista_variables()?);
        Ok(prod)
    }

    pub fn declaracion(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Declaracion);
        self.push_token_if(&TokenType::Tipo, &mut prod)?;
        prod.push_node(self.lista_variables()?);
        Ok(prod)
    }

    pub fn sig_declaraciones(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::SigDeclaraciones);
        if let TokenType::Tipo = self.last_token.token_type {
            prod.push_node(self.declaracion()?);
            self.push_token_if(&TokenType::Semicolon, &mut prod)?;
            prod.push_node(self.sig_declaraciones()?);
        }
        Ok(prod)
    }

    pub fn declaraciones(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Declaraciones);
        prod.push_node(self.declaracion()?);
        self.push_token_if(&TokenType::Semicolon, &mut prod)?;
        prod.push_node(self.sig_declaraciones()?);
        Ok(prod)
    }

    pub fn sig_ordenes(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::SigOrdenes);
        if self.is_orden() {
            prod.push_node(self.orden()?);
            self.push_token_if(&TokenType::Semicolon, &mut prod)?;
            prod.push_node(self.sig_ordenes()?);
        }
        Ok(prod)
    }

    pub fn sig_condicion(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::SigCondicion);
        if let Ok(_) = self.push_token_if(&TokenType::Else, &mut prod) {
            prod.push_node(self.ordenes()?);
        }
        self.push_token_if(&TokenType::End, &mut prod)?;
        return Ok(prod);
    }

    pub fn numeros(&mut self) -> SintacticResult {
        if let TokenType::Entero | TokenType::Real = self.last_token.token_type {
            let mut prod = Production::new(ProductionType::Numeros);
            prod.push_leaf(self.last_token.clone());
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
        if let Ok(_) = self.push_token_if(&TokenType::Id, &mut prod) {
            return Ok(prod);
        }
        prod.push_node(self.numeros()?);
        Ok(prod)
    }

    pub fn comparacion(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Comparacion);
        prod.push_node(self.operador()?);
        self.push_token_if(&TokenType::OperadorCondicion, &mut prod)?;
        prod.push_node(self.operador()?);
        Ok(prod)
    }

    pub fn condicion(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Condicion);
        self.push_token_if(&TokenType::If, &mut prod)?;
        self.push_token_if(&TokenType::ParentesisAbierto, &mut prod)?;
        prod.push_node(self.comparacion()?);
        self.push_token_if(&TokenType::ParentesisCerrado, &mut prod)?;
        prod.push_node(self.ordenes()?);
        prod.push_node(self.sig_condicion()?);
        Ok(prod)
    }

    pub fn bucle_while(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::BucleWhile);
        self.push_token_if(&TokenType::While, &mut prod)?;
        self.push_token_if(&TokenType::ParentesisAbierto, &mut prod)?;
        prod.push_node(self.comparacion()?);
        self.push_token_if(&TokenType::ParentesisCerrado, &mut prod)?;
        prod.push_node(self.ordenes()?);
        self.push_token_if(&TokenType::ParentesisCerrado, &mut prod)?;
        Ok(prod)
    }

    pub fn factor(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Factor);
        if let Ok(_) = self.push_token_if(&TokenType::ParentesisAbierto, &mut prod) {
            prod.push_node(self.expresion_arit()?);
            self.push_token_if(&TokenType::ParentesisCerrado, &mut prod)?;
            return Ok(prod);
        }
        prod.push_node(self.operador()?);
        Ok(prod)
    }

    pub fn rest_term(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::RestTerm);
        if let Ok(_) = self.push_token_if(&TokenType::OperadorAritB, &mut prod) {
            prod.push_node(self.factor()?);
            prod.push_node(self.rest_term()?);
        }
        Ok(prod)
    }

    pub fn termino(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Term);
        prod.push_node(self.factor()?);
        prod.push_node(self.rest_term()?);
        Ok(prod)
    }

    pub fn rest_expr(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::RestExp);
        if let Ok(_) = self.push_token_if(&TokenType::OperadorAritA, &mut prod) {
            prod.push_node(self.termino()?);
            prod.push_node(self.rest_expr()?);
        }
        Ok(prod)
    }

    pub fn expresion_arit(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::ExpresionArit);
        prod.push_node(self.termino()?);
        prod.push_node(self.rest_expr()?);
        Ok(prod)
    }

    pub fn asignar(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Asignar);
        self.push_token_if(&TokenType::Id, &mut prod)?;
        self.push_token_if(&TokenType::OperadorAsig, &mut prod)?;
        prod.push_node(self.expresion_arit()?);
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
                "if, while o una asignación",
            )),
        }?;
        let mut prod = Production::new(ProductionType::Orden);
        prod.push_node(content);
        Ok(prod)
    }

    pub fn ordenes(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Ordenes);
        prod.push_node(self.orden()?);
        self.push_token_if(&TokenType::Semicolon, &mut prod)?;
        prod.push_node(self.sig_ordenes()?);
        Ok(prod)
    }

    pub fn programa(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Programa);
        self.push_token_if(&TokenType::Begin, &mut prod)?;
        prod.push_node(self.declaraciones()?);
        prod.push_node(self.ordenes()?);
        self.push_token_if(&TokenType::End, &mut prod)?;
        Ok(prod)
    }

    pub fn analize(&'a mut self) -> SintacticResult {
        self.next_token();
        let production = self.programa()?;
        self.is_last(&TokenType::EOF)?;
        Ok(production)
    }
}
