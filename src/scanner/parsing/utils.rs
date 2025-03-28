use super::structure::{Kind, Parts};

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
