use std::fs;

use crate::scanner::parsing::utils::{get_action, is_a_group, is_action, quant};

use super::{
    structure::{RegularExpression, ScanParser, Token},
    utils::{handle_token, is_a_char, is_a_class, skip_to_nl},
};

impl ScanParser {
    pub fn parse(scan_path: &str) {
        let content =
            fs::read_to_string(&scan_path).expect("Should have been able to read the file");

        let mut chars = content.chars().peekable();

        let mut exprs = RegularExpression::new();

        while let Some(c) = chars.next() {
            match c {
                '(' => handle_token(&mut chars, is_a_group, &mut exprs),
                '[' => handle_token(&mut chars, is_a_class, &mut exprs),
                '"' => handle_token(&mut chars, is_a_char, &mut exprs),
                ' ' => match is_action(&mut chars.clone()) {
                    true => get_action(&mut chars, &mut exprs),
                    false => {}
                },
                '+' | '*' | '?' => quant(c, &mut exprs),
                _ => {}
            }
        }

        println!("exprs => {}", exprs);

        //println!("content : {content}");
    }
}
