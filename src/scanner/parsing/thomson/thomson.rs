use std::collections::HashMap;

use crate::scanner::parsing::tokenizer::structure::{ExprsLst, Kind, Token};

use super::structure::{Transition, NFA};

fn build_nfa(token: Token) -> NFA {
    let mut nfa = NFA::new();
    nfa.add_state(1, true);
    nfa.add_transition(0, 1, Some(token));
    nfa
}

fn merge_nfa(a: NFA, b: NFA) -> NFA {
    let mut nfa = NFA::new();
    let mut map: HashMap<usize, Transition> = HashMap::new();
    let mut index: usize = 0;
    let mut bp: usize = 0;

    a.display();
    b.display();

    println!("-----------------------------");

    for (i, t) in a.transitions.iter().enumerate() {
        map.insert(i, t.clone());
        index = i;
        bp = i;
    }
    for (i, t) in b.transitions.iter().enumerate() {
        map.insert(index + 1, t.clone());
        index += i;
    }
    println!("DEBUG => map len : {}", map.len());
    for (i, t) in map.iter() {
        if *i >= bp {
            nfa.add_state(i + t.to, false);
            nfa.add_transition(t.from + i, t.to + i, t.symbol.clone());
        } else {
            nfa.add_state(t.to, false);
            nfa.add_transition(t.from, t.to, t.symbol.clone());
        }
    }

    nfa.display();
    println!("=============================");
    nfa
}

pub fn from_postfix_to_nfa(exprs: ExprsLst) -> NFA {
    let mut nfas: Vec<NFA> = Vec::new();
    for expr in exprs.exprs {
        for token in expr.tokens {
            match token.kind {
                Kind::Char => nfas.push(build_nfa(token)),
                Kind::Concat => {
                    if let Some(r) = nfas.pop() {
                        if let Some(l) = nfas.pop() {
                            nfas.push(merge_nfa(l, r));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if let Some(nfa) = nfas.last() {
        //nfa.display();
        println!("DEBUG => {:?}", nfas);
    }
    todo!()
}
