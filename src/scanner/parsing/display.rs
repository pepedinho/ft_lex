use std::fmt;

use crate::scanner::parsing::structure::Kind;

use super::structure::{Parts, Quant, RegularExpression};

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
            Kind::Quantifier(q) => write!(f, "Quantifier({})", q), // On affiche le Quantifier
            Kind::Anchor => write!(f, "Anchor"),
            Kind::Or => write!(f, "Or"),
            Kind::None => write!(f, "None"),
        }
    }
}

impl fmt::Display for Parts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}: \"{}\"]~[{}] ", self.kind, self.content, self.quant)
    }
}

impl fmt::Display for RegularExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parts_str: Vec<String> = self.parts.iter().map(|p| format!("{}", p)).collect();
        write!(
            f,
            "RegularExpression:\n- Content: \"{}\"\n- Parts:\n  {}",
            self.content,
            parts_str.join("\n  ")
        )
    }
}
