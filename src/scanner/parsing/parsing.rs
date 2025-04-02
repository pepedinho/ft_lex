use std::fs;

use crate::scanner::parsing::{
    structure::Kind,
    utils::{escape_char, get_action, is_a_class, is_action, quant, quotes_treatment},
};

use super::structure::{ExprsLst, RegularExpression, ScanParser, Token};

impl ScanParser {
    pub fn parse(scan_path: &str) {
        let content =
            fs::read_to_string(&scan_path).expect("Should have been able to read the file");

        let mut chars = content.chars().peekable();
        let mut exprs = RegularExpression::new();
        let mut list = ExprsLst::new();

        while let Some(mut c) = chars.next() {
            match c {
                '(' => exprs.tokens.push(Token::new(c, Kind::OpenP)),
                ')' => exprs.tokens.push(Token::new(c, Kind::CloseP)),
                '[' => {
                    exprs.content.push(c);
                    let tmp = is_a_class(&mut chars, &mut exprs);
                    exprs.content.push_str(&tmp);
                    continue;
                }
                '"' => {
                    exprs.content.push(c);
                    let tmp = quotes_treatment(&mut chars, &mut exprs);
                    exprs.content.push_str(&tmp);
                }
                ' ' => match is_action(&mut chars.clone()) {
                    true => {
                        get_action(&mut chars, &mut exprs);
                        list.append(exprs);
                        exprs = RegularExpression::new();
                    }
                    false => {}
                },
                '|' => exprs.tokens.push(Token::new(c, Kind::Or)),
                '+' | '*' | '?' => quant(c, &mut exprs),
                '\\' => {
                    if let Some(n_c) = escape_char(&mut chars, &mut exprs) {
                        c = n_c;
                    }
                }
                '\n' | '%' | '\r' => {}
                _ => exprs.tokens.push(Token::new(c, Kind::Char)),
            }
            exprs.content.push(c);
        }

        println!("{}", list);
        // for token in exprs.tokens {
        //     println!("{}", token);
        // }
        // println!("exprs => {}", exprs);

        //println!("content : {content}");
    }
}
