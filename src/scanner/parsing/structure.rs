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
pub struct Parts {
    pub content: String,
    pub kind: Kind,
    pub quant: Kind,
}

#[derive(Debug)]
pub struct RegularExpression {
    pub content: String,
    pub parts: Vec<Parts>,
}

#[derive(Debug)]
pub struct ExpressionTree {
    pub expr: Vec<RegularExpression>,
}

impl Parts {
    pub fn new(content: String, kind: Kind) -> Self {
        Parts {
            content,
            kind,
            quant: Kind::None,
        }
    }
    pub fn add_quant(&mut self, quant: Kind) {
        self.quant = quant;
    }
}

impl RegularExpression {
    pub fn new() -> Self {
        RegularExpression {
            content: String::new(),
            parts: Vec::new(),
        }
    }
}
