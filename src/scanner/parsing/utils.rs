use std::str::Chars;

use super::structure::{Kind, Parts, RegularExpression};

pub fn handle_structure<I, F>(chars: &mut I, parser: F, exprs: &mut RegularExpression)
where
    I: Iterator<Item = char>,
    F: Fn(&mut I) -> Result<Parts, String>,
{
    match parser(chars) {
        Ok(p) => {
            exprs.parts.push(p);
        }
        Err(e) => {
            println!("Error : {}", e);
        }
    }
}

pub fn is_a_group<I>(chars: &mut I) -> Result<Parts, String>
where
    I: Iterator<Item = char>,
{
    let mut content = String::new();

    while let Some(c) = chars.next() {
        match c {
            ')' => {
                return Ok(Parts::new(content, Kind::Groupe));
            }
            '"' => {
                return Err("In some quotes".to_string());
            }
            _ => content.push(c),
        }
    }
    return Err("Excepted ')'".to_string());
}

pub fn is_a_class<I>(chars: &mut I) -> Result<Parts, String>
where
    I: Iterator<Item = char>,
{
    let mut content = String::new();

    while let Some(c) = chars.next() {
        match c {
            ']' => {
                return Ok(Parts::new(content, Kind::Classe));
            }
            '"' => {
                return Err("In some quotes".to_string());
            }
            _ => content.push(c),
        }
    }
    return Err("Excepted ']'".to_string());
}

pub fn is_a_char<I>(chars: &mut I) -> Result<Parts, String>
where
    I: Iterator<Item = char>,
{
    let mut content = String::new();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                return Ok(Parts::new(content, Kind::Char));
            }
            _ => content.push(c),
        }
    }
    return Err("Excepted '\"'".to_string());
}

pub fn skip_to_nl<I>(chars: &mut I)
where
    I: Iterator<Item = char>,
{
    while let Some(next_c) = chars.next() {
        if next_c == '\n' {
            break;
        }
    }
}

pub fn quant(c: char, exprs: &mut RegularExpression) {
    if let Some(last_part) = exprs.parts.last_mut() {
        match c {
            '+' => {
                last_part.add_quant(Kind::Quantifier(super::structure::Quant::Plus));
            }
            '*' => last_part.add_quant(Kind::Quantifier(super::structure::Quant::Star)),
            '?' => last_part.add_quant(Kind::Quantifier(super::structure::Quant::Interrogation)),
            _ => {}
        }
        //println!("LAST PART => {} | c => {}", last_part, c);
    }
}
