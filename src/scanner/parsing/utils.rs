use std::{char, str::Chars};

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

pub fn is_a_class(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    expr: &mut RegularExpression,
) -> String {
    let mut previous_char: Option<char> = None;
    let mut content = String::new();
    while let Some(mut c) = chars.next() {
        if let Some(_p) = previous_char {
            if c != ']' {
                expr.tokens.push(Token::new('|', Kind::Or));
            }
        }
        match c {
            ']' => {
                content.push(c);
                break;
            }
            '-' => {
                if let Some(p_char) = previous_char {
                    match p_char {
                        '\\' | '\0' => {
                            expr.tokens.push(Token::new(c, Kind::Char));
                            content.push(c);
                        }
                        _ => {
                            content.push(c);
                            let mut ite = (p_char as u8 + 1) as char;
                            if let Some(n_char) = chars.next() {
                                //TODO: gerer si b_char > n_char
                                while ite <= n_char {
                                    expr.tokens.push(Token::new(ite, Kind::Char));
                                    if ite != n_char {
                                        expr.tokens.push(Token::new('|', Kind::Or));
                                    }
                                    ite = (ite as u8 + 1) as char;
                                }
                                c = n_char;
                                content.push(c);
                            }
                        }
                    }
                }
            }
            _ => {
                expr.tokens.push(Token::new(c, Kind::Char));
                content.push(c);
            }
        }
        previous_char = Some(c);
    }
    content
}

pub fn escape_char<I>(chars: &mut I, expr: &mut RegularExpression) -> Option<char>
where
    I: Iterator<Item = char>,
{
    if let Some(n_c) = chars.next() {
        match n_c {
            'n' => expr.tokens.push(Token::new('\n', Kind::Char)),
            't' => expr.tokens.push(Token::new('\t', Kind::Char)),
            'r' => expr.tokens.push(Token::new('\r', Kind::Char)),
            _ => expr.tokens.push(Token::new(n_c, Kind::Char)),
        }
        return Some(n_c);
    }
    None
}

pub fn quotes_treatment<I>(chars: &mut I, expr: &mut RegularExpression) -> String
where
    I: Iterator<Item = char>,
{
    let mut content = String::new();

    while let Some(mut c) = chars.next() {
        match c {
            '"' => {
                break;
            }
            '\\' => {
                if let Some(n_c) = escape_char(chars, expr) {
                    c = n_c;
                }
            }
            _ => expr.tokens.push(Token::new(c, Kind::Char)),
        }
        content.push(c);
    }
    content
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
