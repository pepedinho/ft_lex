use std::fmt;

use crate::scanner::parsing::structure::Kind;

use super::structure::{ExprsLst, Quant, RegularExpression, Token};

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
            Kind::OpenP => write!(f, "OpenP"),
            Kind::CloseP => write!(f, "CloseP"),
            Kind::OpenB => write!(f, "OpenB"),
            Kind::CloseB => write!(f, "CloseB"),
            Kind::Quotes => write!(f, "Quote"),
            Kind::Quantifier(q) => write!(f, "Quantifier({})", q),
            Kind::Anchor => write!(f, "Anchor"),
            Kind::Or => write!(f, "Or"),
            Kind::None => write!(f, "None"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}: \"{}\"]\n", self.kind, self.content)
    }
}

impl fmt::Display for RegularExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parts_str: Vec<String> = self.tokens.iter().map(|p| format!("{}", p)).collect();
        write!(
            f,
            "RegularExpression:\n- Content: \"{}\"\n- Token:\n  {}\n\t\t Action: {}",
            self.content,
            parts_str.join("\n  "),
            self.action,
        )
    }
}

impl fmt::Display for ExprsLst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let exprs_str: Vec<String> = self.exprs.iter().map(|expr| format!("{}", expr)).collect();
        write!(
            f,
            "ExprsLst:\n{}",
            exprs_str.join("\n-----------------------------------------------\n")
        )
    }
}
