use std::fmt;

use crate::scanner::parsing::tokenizer::structure::{Kind, RepCases};

use super::structure::{ExprsLst, Quant, RegularExpression, Repetition, Token};

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

impl fmt::Display for Repetition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vals: Vec<String> = self.values.iter().map(|p| format!("{}", p)).collect();
        write!(f, "[ranges : ({}) | case : {}]", vals.join(","), self.case)
    }
}

impl fmt::Display for RepCases {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rep_str = match self {
            RepCases::Between => "Between",
            RepCases::Exact => "Exact",
            RepCases::AtLeast => "AtLeast",
        };
        write!(f, "{}", rep_str)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Char => write!(f, "Char"),
            Kind::OpenP => write!(f, "OpenP"),
            Kind::CloseP => write!(f, "CloseP"),
            Kind::If => write!(f, "If"),
            Kind::Repetition(q) => write!(f, "Repetition({})", q),
            Kind::Quantifier(q) => write!(f, "Quantifier({})", q),
            Kind::Anchor => write!(f, "Anchor"),
            Kind::Or => write!(f, "Or"),
            Kind::None => write!(f, "None"),
            Kind::Concat => write!(f, "Concat"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}: '{}']\n", self.kind, self.content)
    }
}

impl fmt::Display for RegularExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parts_str: Vec<String> = self.tokens.iter().map(|p| format!("{}", p)).collect();
        write!(
            f,
            "RegularExpression:\n- Content: \"{}\"\n- Token:\n{}\n- Action:\n{}",
            self.content,
            parts_str.join(""),
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
