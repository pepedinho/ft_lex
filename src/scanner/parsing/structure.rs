use std::{collections::btree_map::Values, fmt};

pub struct ScanParser {
    pub content: String,
    pub filename: String,
    pub count: Counter,
}

pub struct Counter {
    pub char: i32,
    pub lines: i32,
}

impl Counter {
    pub fn new() -> Self {
        Counter { char: 1, lines: 1 }
    }
}

#[derive(Debug, PartialEq)]
pub enum Quant {
    Plus,
    Star,
    Interrogation,
    Brackets,
}

#[derive(Debug, PartialEq)]
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
    Concat,
    Repetition(Repetition),
    None,
}

#[derive(Debug, PartialEq)]
pub enum RepCases {
    Exact,
    AtLeast,
    Between,
}

#[derive(Debug, PartialEq)]
pub struct Repetition {
    pub case: RepCases,
    pub values: Vec<i32>,
}

impl Repetition {
    pub fn new(values: Vec<i32>, case: RepCases) -> Self {
        Repetition { values, case }
    }
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
    pub fn append_token(&mut self, token: Token) {
        if let Some(last) = self.tokens.last() {
            if last.kind == Kind::Char && token.kind == Kind::Char {
                self.tokens.push(Token::new('²', Kind::Concat));
                self.tokens.push(token);
                return;
            }
            self.tokens.push(token);
            return;
        }
        self.tokens.push(token);
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
