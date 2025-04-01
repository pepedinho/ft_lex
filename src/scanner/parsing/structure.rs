use std::fmt;

pub struct ScanParser {
    pub content: String,
}

#[derive(Debug)]
pub enum Quant {
    Plus,
    Star,
    Interrogation,
    Brackets,
}

#[derive(Debug)]
pub enum Kind {
    Char,              // ""
    Groupe,            //()
    Classe,            //[]
    Quantifier(Quant), //*, +, ?, {}
    Anchor,            // ^, $
    Or,                // |
    None,
}

#[derive(Debug)]
pub struct Token {
    pub content: String,
    pub kind: Kind,
    pub quant: Kind,
    pub action: String,
}

#[derive(Debug)]
pub struct RegularExpression {
    pub content: String,
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct ExpressionTree {
    pub expr: Vec<RegularExpression>,
}

impl Token {
    pub fn new(content: String, kind: Kind) -> Self {
        Token {
            content,
            kind,
            action: String::new(),
            quant: Kind::None,
        }
    }
    pub fn add_quant(&mut self, quant: Kind) {
        self.quant = quant;
    }
    pub fn add_action(&mut self, action: String) {
        self.action = action;
    }
}

impl RegularExpression {
    pub fn new() -> Self {
        RegularExpression {
            content: String::new(),
            tokens: Vec::new(),
        }
    }
}
