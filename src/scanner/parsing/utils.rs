use std::str::Chars;

use super::structure::{Kind, RegularExpression, Token};

pub fn handle_token<I, F>(chars: &mut I, parser: F, exprs: &mut RegularExpression)
where
    I: Iterator<Item = char>,
    F: Fn(&mut I) -> Result<Token, String>,
{
    match parser(chars) {
        Ok(p) => {
            exprs.tokens.push(p);
        }
        Err(e) => {
            println!("Error : {}", e);
        }
    }
}

// pub fn is_a_group<I>(chars: &mut I) -> Result<Token, String>
// where
//     I: Iterator<Item = char>,
// {
//     let mut content = String::new();
//
//     while let Some(c) = chars.next() {
//         match c {
//             ')' => {
//                 return Ok(Token::new(c, Kind::Groupe));
//             }
//             '"' => {
//                 return Err("In some quotes".to_string());
//             }
//             _ => content.push(c),
//         }
//     }
//     return Err("Excepted ')'".to_string());
// }

// pub fn is_a_class<I>(chars: &mut I) -> Result<Token, String>
// where
//     I: Iterator<Item = char>,
// {
//     let mut content = String::new();
//
//     while let Some(c) = chars.next() {
//         match c {
//             ']' => {
//                 return Ok(Token::new(c, Kind::Classe));
//             }
//             '"' => {
//                 return Err("In some quotes".to_string());
//             }
//             _ => content.push(c),
//         }
//     }
//     return Err("Excepted ']'".to_string());
// }

pub fn is_a_char<I>(chars: &mut I) -> Result<Token, String>
where
    I: Iterator<Item = char>,
{
    let mut content = String::new();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                return Ok(Token::new(c, Kind::Char));
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

pub fn is_action<I>(chars: &mut I) -> bool
where
    I: Iterator<Item = char>,
{
    while let Some(next_c) = chars.next() {
        if next_c != ' ' {
            if next_c != '|' && next_c != '\\' {
                return true;
            }
            return false;
        }
    }
    true
}

pub fn get_action<I>(chars: &mut I, exprs: &mut RegularExpression)
where
    I: Iterator<Item = char>,
{
    let mut action = String::new();
    while let Some(next_c) = chars.next() {
        if next_c == ';' {
            break;
        }
        if next_c != ' ' {
            action.push(next_c);
        }
    }
    exprs.action = action;
}

pub fn quant(c: char, exprs: &mut RegularExpression) {
    match c {
        '+' => {
            exprs.tokens.push(Token::new(
                c,
                Kind::Quantifier(super::structure::Quant::Plus),
            ));
        }
        '*' => {
            exprs.tokens.push(Token::new(
                c,
                Kind::Quantifier(super::structure::Quant::Star),
            ));
        }
        '?' => {
            exprs.tokens.push(Token::new(
                c,
                Kind::Quantifier(super::structure::Quant::Interrogation),
            ));
        }
        _ => {}
    }
    //println!("LAST PART => {} | c => {}", last_part, c);
}
