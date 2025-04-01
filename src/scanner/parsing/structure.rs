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
    Char,              // abc
    Quotes,            // ""
    OpenP,             //(
    CloseP,            //)
    OpenB,             //[
    CloseB,            //]
    Quantifier(Quant), //*, +, ?, {}
    Anchor,            // ^, $
    Or,                // |
    None,
}

#[derive(Debug)]
pub struct Token {
    pub content: char,
    pub kind: Kind,
}

#[derive(Debug)]
pub struct RegularExpression {
    pub content: String,
    pub tokens: Vec<Token>,
    pub action: String,
}

#[derive(Debug)]
pub struct ExprsLst {
    pub exprs: Vec<RegularExpression>,
}

impl Token {
    pub fn new(content: char, kind: Kind) -> Self {
        Token { content, kind }
    }
}

impl RegularExpression {
    pub fn new() -> Self {
        RegularExpression {
            content: String::new(),
            tokens: Vec::new(),
            action: String::new(),
        }
    }
}

impl ExprsLst {
    pub fn new() -> Self {
        ExprsLst { exprs: Vec::new() }
    }
    pub fn append(&mut self, expr: RegularExpression) {
        self.exprs.push(expr);
    }
}
