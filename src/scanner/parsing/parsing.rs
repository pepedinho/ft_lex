use std::fs;

use crate::scanner::parsing::{
    structure::Kind,
    utils::{get_action, is_action, quant},
};

use super::structure::{ExprsLst, RegularExpression, ScanParser, Token};

impl ScanParser {
    pub fn parse(scan_path: &str) {
        let content =
            fs::read_to_string(&scan_path).expect("Should have been able to read the file");

        let mut chars = content.chars().peekable();

        let mut exprs = RegularExpression::new();

        let mut list = ExprsLst::new();

        while let Some(c) = chars.next() {
            match c {
                '(' => exprs.tokens.push(Token::new(c, Kind::OpenP)),
                ')' => exprs.tokens.push(Token::new(c, Kind::CloseP)),
                '[' => exprs.tokens.push(Token::new(c, Kind::OpenB)),
                ']' => exprs.tokens.push(Token::new(c, Kind::CloseB)),
                '"' => exprs.tokens.push(Token::new(c, Kind::Quotes)),
                ' ' => match is_action(&mut chars.clone()) {
                    true => {
                        get_action(&mut chars, &mut exprs);
                        list.append(exprs);
                        exprs = RegularExpression::new();
                    }
                    false => exprs.tokens.push(Token::new(c, Kind::Char)),
                },
                '+' | '*' | '?' => quant(c, &mut exprs),
                _ => {}
            }
        }

        println!("{}", list);
        // for token in exprs.tokens {
        //     println!("{}", token);
        // }
        // println!("exprs => {}", exprs);

        //println!("content : {content}");
    }
}
