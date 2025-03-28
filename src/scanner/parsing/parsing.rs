use std::fs;

use crate::scanner::parsing::utils::is_a_group;

use super::structure::ScanParser;

impl ScanParser {
    pub fn parse(scan_path: &str) {
        let content =
            fs::read_to_string(&scan_path).expect("Should have been able to read the file");

        let mut chars = content.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '(' => {
                    let part = match is_a_group(&mut chars) {
                        Ok(p) => p,
                        Err(e) => {
                            println!("Error : {}", e);
                            return;
                        }
                    };
                    println!("part => {:?}", part);
                }
                ' ' => {
                    while let Some(next_c) = chars.next() {
                        if next_c == '\n' {
                            break;
                        }
                    }
                }
                _ => {}
            }

            if c == '(' {
                let part = match is_a_group(&mut chars) {
                    Ok(p) => p,
                    Err(e) => {
                        println!("Error : {}", e);
                        return;
                    }
                };
                println!("part => {:?}", part);
            }
        }

        //println!("content : {content}");
    }
}
