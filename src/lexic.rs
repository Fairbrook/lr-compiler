use std::str::Chars;

use crate::token::{Token, TokenType};

pub struct LexicAnalyzer<'a> {
    pub input: String,
    pub current: char,
    pub iter: Chars<'a>,
    pub current_line: usize,
    pub current_col: usize,
}

impl<'a> LexicAnalyzer<'a> {
    pub fn new(input: &'a str) -> LexicAnalyzer<'a> {
        let mut iter = input.chars();
        let current = if let Some(character) = iter.next() {
            character
        } else {
            '\0'
        };
        LexicAnalyzer {
            iter,
            input: input.to_string(),
            current,
            current_line: 1,
            current_col: 1,
        }
    }

    pub fn next_char(&mut self) -> char {
        let next = if let Some(character) = self.iter.next() {
            character
        } else {
            '\0'
        };
        if next != '\0' {
            self.current_col += 1;
            if next == '\n' {
                self.current_line += 1;
                self.current_col = 0;
            }
        }
        self.current = next;
        next
    }

    pub fn skip_empty(&mut self) {
        while let '\n' | '\t' | '\r' | ' ' = self.current {
            self.next_char();
        }
    }

    pub fn single_character_token(&mut self) -> Option<Token> {
        let token_type = match self.current {
            ',' => Some(TokenType::Coma),
            ';' => Some(TokenType::Semicolon),
            '(' => Some(TokenType::ParentesisAbierto),
            ')' => Some(TokenType::ParentesisCerrado),
            '.' => Some(TokenType::Punto),
            _ => None,
        };
        if let Some(token_type) = token_type {
            let token = Token {
                token_type,
                lexeme: self.current.to_string(),
                line: self.current_line as u32,
                col: self.current_col as u32,
            };
            self.next_char();
            return Some(token);
        }
        None
    }

    pub fn real_number(&mut self, number: Token) -> Token {
        let rest = self.number();
        return Token {
            token_type: TokenType::Real,
            lexeme: format!("{}.{}", number.lexeme, rest.lexeme),
            line: number.line,
            col: number.col,
        };
    }

    pub fn number(&mut self) -> Token {
        let mut token = Token {
            lexeme: String::new(),
            col: self.current_col as u32,
            line: self.current_line as u32,
            token_type: TokenType::Entero,
        };
        while let '0'..='9' = self.current {
            token.lexeme.push(self.current.clone());
            self.next_char();
        }
        if self.current == '.' {
            if let '0'..='9' = self.next_char() {
                return self.real_number(token);
            }
        }
        token
    }

    pub fn operator(&mut self) -> Token {
        let token = Token {
            token_type: TokenType::OperadorArit,
            lexeme: self.current.to_string(),
            line: self.current_line as u32,
            col: self.current_col as u32,
        };
        self.next_char();
        token
    }

    pub fn logic_operator(&mut self) -> Token {
        let mut token = Token {
            token_type: TokenType::OperadorCondicion,
            lexeme: self.current.to_string(),
            line: self.current_line as u32,
            col: self.current_col as u32,
        };
        let current = self.current.clone();
        self.next_char();
        if let '>' | '<' = current {
            if self.current == '=' {
                token.lexeme.push(self.current);
                self.next_char();
            }
        }
        if current == '<' && self.current == '>' {
            token.lexeme.push(self.current);
            self.next_char();
        }
        token
    }

    pub fn asign_operator(&mut self) -> Token {
        let mut token = Token {
            token_type: TokenType::OperadorAsig,
            lexeme: self.current.to_string(),
            line: self.current_line as u32,
            col: self.current_col as u32,
        };
        if self.next_char() == '=' {
            token.lexeme.push(self.current.clone());
            return token;
        }
        Token {
            token_type: TokenType::Unknown,
            lexeme: self.current.to_string(),
            line: self.current_line as u32,
            col: self.current_col as u32,
        }
    }

    pub fn identifier(&mut self) -> Token {
        let mut token = Token {
            token_type: TokenType::Id,
            lexeme: String::new(),
            line: self.current_line as u32,
            col: self.current_col as u32,
        };
        while let '0'..='9' | 'a'..='z' | 'A'..='Z' = self.current {
            token.lexeme.push(self.current);
            self.next_char();
        }
        token
    }

    pub fn reserved_word(id_token: &Token) -> Option<Token> {
        let token_type = match id_token.lexeme.as_str() {
            "real" | "entero" => Some(TokenType::Tipo),
            "begin" => Some(TokenType::Begin),
            "end" => Some(TokenType::End),
            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "while" => Some(TokenType::While),
            "endwhile" => Some(TokenType::Endwhile),
            _ => None,
        };
        if let Some(token_type) = token_type {
            let mut token = id_token.clone();
            token.token_type = token_type;
            return Some(token);
        }
        None
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_empty();
        if let Some(token) = self.single_character_token() {
            return token;
        }

        match self.current {
            '0'..='9' => self.number(),
            ':' => self.asign_operator(),
            '+' | '-' | '*' | '/' => self.operator(),
            '=' | '<' | '>' => self.logic_operator(),
            'a'..='z' | 'A'..='Z' => {
                let token = self.identifier();
                if let Some(reserved) = LexicAnalyzer::reserved_word(&token) {
                    return reserved;
                }
                token
            }
            '\0' => Token {
                token_type: TokenType::EOF,
                lexeme: self.current.to_string(),
                line: self.current_line as u32,
                col: self.current_col as u32,
            },

            _ => {
                let token = Token {
                    token_type: TokenType::Unknown,
                    lexeme: self.current.to_string(),
                    line: self.current_line as u32,
                    col: self.current_col as u32,
                };
                self.next_char();
                token
            }
        }
    }
}
