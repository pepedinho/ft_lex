use std::fmt;

use crate::scanner::parsing::structure::Kind;

use super::structure::{Quant, RegularExpression, Token};

impl fmt::Display for Quant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let quant_str = match self {
            Quant::Plus => "+",
            Quant::Star => "*",
            Quant::Interrogation => "?",
            Quant::Brackets => "{}",
        };
        write!(f, "{}", quant_str)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Char => write!(f, "Char"),
            Kind::Groupe => write!(f, "Group"),
            Kind::Classe => write!(f, "Class"),
            Kind::Quantifier(q) => write!(f, "Quantifier({})", q),
            Kind::Anchor => write!(f, "Anchor"),
            Kind::Or => write!(f, "Or"),
            Kind::None => write!(f, "None"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}: \"{}\"]~[{}] \n\t\t(action) -> [{}]",
            self.kind, self.content, self.quant, self.action
        )
    }
}

impl fmt::Display for RegularExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parts_str: Vec<String> = self.tokens.iter().map(|p| format!("{}", p)).collect();
        write!(
            f,
            "RegularExpression:\n- Content: \"{}\"\n- Token:\n  {}",
            self.content,
            parts_str.join("\n  ")
        )
    }
}
