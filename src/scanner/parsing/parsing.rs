use std::fs;

use crate::scanner::parsing::utils::{is_a_group, quant};

use super::{
    structure::{Parts, RegularExpression, ScanParser},
    utils::{handle_structure, is_a_char, is_a_class, skip_to_nl},
};

impl ScanParser {
    pub fn parse(scan_path: &str) {
        let content =
            fs::read_to_string(&scan_path).expect("Should have been able to read the file");

        let mut chars = content.chars().peekable();

        let mut exprs = RegularExpression::new();

        while let Some(c) = chars.next() {
            match c {
                '(' => handle_structure(&mut chars, is_a_group, &mut exprs),
                '[' => handle_structure(&mut chars, is_a_class, &mut exprs),
                '"' => handle_structure(&mut chars, is_a_char, &mut exprs),
                ' ' => skip_to_nl(&mut chars),
                '+' | '*' | '?' => quant(c, &mut exprs),
                _ => {}
            }
        }

        println!("exprs => {}", exprs);

        //println!("content : {content}");
    }
}
