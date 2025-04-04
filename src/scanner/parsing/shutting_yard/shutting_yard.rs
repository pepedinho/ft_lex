use crate::scanner::parsing::tokenizer::structure::{
    ExprsLst, Kind, Quant, RegularExpression, Token,
};

impl ExprsLst {
    pub fn to_postfix(&mut self) {
        let mut op_stack: Vec<Token> = Vec::new();
        let mut new: Vec<RegularExpression> = Vec::new();
        //let mut trash: Vec<Token>;
        for expr in &self.exprs {
            let mut exprs = RegularExpression::new();
            for token in expr.tokens.clone() {
                match token.kind {
                    Kind::Char => exprs.tokens.push(token),
                    Kind::Quantifier(_) => {
                        if let Some(last) = op_stack.last() {
                            if matches!(last.kind, Kind::Quantifier(_)) {
                                if let Some(op) = op_stack.pop() {
                                    exprs.tokens.push(op);
                                }
                            }
                        }
                        op_stack.push(token);
                    }
                    Kind::Concat => {
                        if let Some(last) = op_stack.last() {
                            match last.kind {
                                Kind::Quantifier(_) | Kind::Concat => {
                                    if let Some(op) = op_stack.pop() {
                                        exprs.tokens.push(op);
                                    }
                                }
                                _ => {}
                            }
                        }
                        op_stack.push(token);
                    }
                    Kind::Or => {
                        if let Some(last) = op_stack.last() {
                            match last.kind {
                                Kind::Quantifier(_) | Kind::Concat | Kind::Or => {
                                    if let Some(op) = op_stack.pop() {
                                        exprs.tokens.push(op);
                                    }
                                }
                                _ => {}
                            }
                        }
                        op_stack.push(token);
                    }
                    Kind::If => {
                        if let Some(last) = op_stack.last() {
                            match last.kind {
                                Kind::Quantifier(_) | Kind::Concat | Kind::Or | Kind::If => {
                                    if matches!(last.kind, Kind::Quantifier(_)) {
                                        if let Some(op) = op_stack.pop() {
                                            exprs.tokens.push(op);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        op_stack.push(token);
                    }
                    Kind::Repetition(_) => {
                        println!("JE SUIS RENTRER => {:?}", &op_stack);
                        if let Some(last) = op_stack.last() {
                            match last.kind {
                                Kind::Quantifier(_)
                                | Kind::Concat
                                | Kind::Or
                                | Kind::If
                                | Kind::Repetition(_) => {
                                    if matches!(last.kind, Kind::Quantifier(_)) {
                                        if let Some(op) = op_stack.pop() {
                                            exprs.tokens.push(op);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        op_stack.push(token);
                    }
                    Kind::OpenP => op_stack.push(token),
                    Kind::CloseP => {
                        if let Some(_last) = op_stack.last() {
                            while let Some(op) = op_stack.pop() {
                                println!("[{}] POP IT => {}", expr.content, &op);
                                match op.kind {
                                    Kind::OpenP => break,
                                    _ => {
                                        exprs.tokens.push(op);
                                    }
                                }
                            }
                            println!(
                                "DBEUEBUEBUDBUDBUEBUEBU => ({:?}) | expr last ({:?})",
                                op_stack.last(),
                                exprs.tokens.last()
                            );
                        }
                    }
                    _ => {}
                }
            }
            while let Some(token) = op_stack.pop() {
                println!("ADD +=> {}", token);
                exprs.tokens.push(token);
            }
            new.push(exprs);
        }
        self.exprs = new;
    }
}
