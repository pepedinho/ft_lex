pub struct ScanParser {
    pub content: String,
}

#[derive(Debug)]
pub enum Kind {
    Char,       // ""
    Groupe,     //()
    Classe,     //[]
    Quantifier, //*, +, ?, {}
    Anchor,     // ^, $
    Or,         // |
}

#[derive(Debug)]
pub struct Parts {
    pub content: String,
    pub kind: Kind,
}

#[derive(Debug)]
pub struct RegularExpression {
    pub content: String,
    pub parts: Vec<Parts>,
}

impl Parts {
    pub fn new(content: String, kind: Kind) -> Self {
        Parts { content, kind }
    }
}
