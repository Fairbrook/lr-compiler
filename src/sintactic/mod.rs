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

    pub fn push_if(
        &mut self,
        token_type: &TokenType,
        production: &mut Production,
    ) -> Result<(), SintacticError> {
        self.is_last(token_type)?;
        production
            .items
            .push(ProductionItem::Leaf(self.last_token.clone()));
        self.next_token();
        Ok(())
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
        self.push_if(&TokenType::Id, &mut prod)?;
        let sig = self.sig_lista_variables()?;
        prod.items.push(ProductionItem::Production(sig));
        Ok(prod)
    }

    pub fn declaracion(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Declaracion);
        self.push_if(&TokenType::Tipo, &mut prod)?;
        let lista = self.lista_variables()?;
        prod.items.push(ProductionItem::Production(lista));
        Ok(prod)
    }

    pub fn sig_declaraciones(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::SigDeclaraciones);
        if let TokenType::Tipo = self.last_token.token_type {
            let declaraciones = self.declaracion()?;
            prod.items.push(ProductionItem::Production(declaraciones));

            self.push_if(&TokenType::Semicolon, &mut prod)?;

            let sig = self.sig_declaraciones()?;
            prod.items.push(ProductionItem::Production(sig));
        }
        Ok(prod)
    }

    pub fn declaraciones(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Declaraciones);
        let declaracion = self.declaracion()?;
        prod.items.push(ProductionItem::Production(declaracion));

        self.push_if(&TokenType::Semicolon, &mut prod)?;

        let sig = self.sig_declaraciones()?;
        prod.items.push(ProductionItem::Production(sig));
        Ok(prod)
    }

    pub fn sig_ordenes(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::SigOrdenes);
        if self.is_orden() {
            let orden = self.orden()?;
            prod.items.push(ProductionItem::Production(orden));

            self.push_if(&TokenType::Semicolon, &mut prod)?;

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
        self.push_if(&TokenType::End, &mut prod)?;

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

        self.push_if(&TokenType::OperadorCondicion, &mut prod)?;

        let operador = self.operador()?;
        prod.items.push(ProductionItem::Production(operador));
        Ok(prod)
    }

    pub fn condicion(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Condicion);

        self.push_if(&TokenType::If, &mut prod)?;
        self.push_if(&TokenType::ParentesisAbierto, &mut prod)?;

        let comparacion = self.comparacion()?;
        prod.items.push(ProductionItem::Production(comparacion));

        self.push_if(&TokenType::ParentesisCerrado, &mut prod)?;

        let ordenes = self.ordenes()?;
        prod.items.push(ProductionItem::Production(ordenes));

        let sig = self.sig_condicion()?;
        prod.items.push(ProductionItem::Production(sig));
        Ok(prod)
    }

    pub fn bucle_while(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::BucleWhile);

        self.push_if(&TokenType::While, &mut prod)?;
        self.push_if(&TokenType::ParentesisAbierto, &mut prod)?;

        let comparacion = self.comparacion()?;
        prod.items.push(ProductionItem::Production(comparacion));

        self.push_if(&TokenType::ParentesisCerrado, &mut prod)?;

        let ordenes = self.ordenes()?;
        prod.items.push(ProductionItem::Production(ordenes));

        self.push_if(&TokenType::ParentesisCerrado, &mut prod)?;
        Ok(prod)
    }

    pub fn factor(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Factor);
        if let TokenType::ParentesisAbierto = self.last_token.token_type {
            prod.items
                .push(ProductionItem::Leaf(self.last_token.clone()));
            self.next_token();

            let exp = self.expresion_arit()?;
            prod.items.push(ProductionItem::Production(exp));

            self.push_if(&TokenType::ParentesisCerrado, &mut prod)?;
            return Ok(prod);
        }

        let operador = self.operador()?;
        prod.items.push(ProductionItem::Production(operador));
        Ok(prod)
    }

    pub fn rest_term(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::RestTerm);
        if let TokenType::OperadorAritB = self.last_token.token_type {
            prod.items
                .push(ProductionItem::Leaf(self.last_token.clone()));
            self.next_token();

            let op = self.factor()?;
            prod.items.push(ProductionItem::Production(op));

            let exp = self.rest_term()?;
            prod.items.push(ProductionItem::Production(exp));
        }
        Ok(prod)
    }

    pub fn termino(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Term);
        let factor = self.factor()?;
        prod.items.push(ProductionItem::Production(factor));

        let rest_term = self.rest_term()?;
        prod.items.push(ProductionItem::Production(rest_term));
        Ok(prod)
    }

    pub fn rest_expr(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::RestExp);
        if let TokenType::OperadorAritA = self.last_token.token_type {
            prod.items
                .push(ProductionItem::Leaf(self.last_token.clone()));
            self.next_token();

            let op = self.termino()?;
            prod.items.push(ProductionItem::Production(op));

            let exp = self.rest_expr()?;
            prod.items.push(ProductionItem::Production(exp));
        }
        Ok(prod)
    }

    pub fn expresion_arit(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::ExpresionArit);
        let op = self.termino()?;
        prod.items.push(ProductionItem::Production(op));

        let exp = self.rest_expr()?;
        prod.items.push(ProductionItem::Production(exp));
        Ok(prod)
    }

    pub fn asignar(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Asignar);
        self.push_if(&TokenType::Id, &mut prod)?;
        self.push_if(&TokenType::OperadorAsig, &mut prod)?;

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

        self.push_if(&TokenType::Semicolon, &mut prod)?;

        let sig = self.sig_ordenes()?;
        prod.items.push(ProductionItem::Production(sig));
        Ok(prod)
    }

    pub fn programa(&mut self) -> SintacticResult {
        let mut prod = Production::new(ProductionType::Programa);
        self.push_if(&TokenType::Begin, &mut prod)?;

        let declaraciones = self.declaraciones()?;
        prod.items.push(ProductionItem::Production(declaraciones));

        let ordenes = self.ordenes()?;
        prod.items.push(ProductionItem::Production(ordenes));

        self.push_if(&TokenType::End, &mut prod)?;
        Ok(prod)
    }

    pub fn analize(&'a mut self) -> SintacticResult {
        self.next_token();
        let production = self.programa()?;
        self.is_last(&TokenType::EOF)?;
        Ok(production)
    }
}
