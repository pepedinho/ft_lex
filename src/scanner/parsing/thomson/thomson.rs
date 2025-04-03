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
    let mut map: Vec<Transition> = Vec::new();
    let mut index: usize = 0;
    let mut bp: usize = 0;

    //println!("a________");
    //a.display();
    //println!("b________");
    //b.display();
    //
    //println!("-----------------------------");

    for (i, t) in a.transitions.iter().enumerate() {
        map.push(t.clone());
        index = i;
        bp = i;
    }
    for (i, t) in b.transitions.iter().enumerate() {
        map.push(t.clone());
        index += i;
    }
    println!("DEBUG => bp = {}", bp);
    for (i, t) in map.iter().enumerate() {
        if i > bp {
            nfa.add_state(i + t.to, false);
            nfa.add_transition(t.from + i, t.to + i, t.symbol.clone());
            //println!(
            //    "({})create [{}] from({}) to ({})",
            //    i,
            //    t.symbol.clone().unwrap().content,
            //    t.from + i,
            //    t.to + i,
            //);
        } else {
            //println!(
            //    "({})[{}] has been stacked",
            //    i,
            //    t.symbol.clone().unwrap().content
            //);
            nfa.add_state(t.to, false);
            nfa.add_transition(t.from, t.to, t.symbol.clone());
        }
    }

    //nfa.display();
    println!("=============================");
    nfa
}

fn sync_and_epsilon(a: &mut NFA, b: &mut NFA) -> Vec<Vec<Transition>> {
    let mut res: Vec<Vec<Transition>> = Vec::new();
    res.push(a.transitions.clone());
    res.push(b.transitions.clone());
    //insert epsilone at start
    res[0].insert(
        0,
        Transition {
            symbol: None,
            from: 0,
            to: 1,
        },
    );
    if let Some(last) = res[0].clone().last() {
        res[1].insert(
            0,
            Transition {
                symbol: None,
                from: 0,
                to: last.to + 3,
            },
        );
    }
    // sync all a stack states
    if let Some(first) = res[0].first() {
        let mut last = first.to;
        for (i, t) in res[0].iter_mut().enumerate() {
            if i > 0 {
                t.from = last;
                t.to = last + 1;
                last += 1;
            }
        }
    }
    // push last epsilone Transition for a
    if let Some(l) = res[0].clone().last() {
        res[0].push(Transition {
            symbol: None,
            from: l.to,
            to: l.to + 1,
        });
    }
    // sync all b stack states
    if let Some(first) = res[1].first() {
        let mut last = first.to;
        for (i, t) in res[1].iter_mut().enumerate() {
            if i > 0 {
                t.from = last;
                t.to = last + 1;
                last += 1;
            }
        }
    }
    if let Some(l) = res[1].clone().last() {
        if let Some(ll) = res[0].clone().last() {
            res[1].push(Transition {
                symbol: None,
                from: l.to,
                to: ll.to,
            });
        }
    }

    res
}

pub fn make_union_between(a: &mut NFA, b: &mut NFA) -> NFA {
    let mut eps = sync_and_epsilon(a, b);
    let mut nfa = NFA::new();

    println!("LEFT");
    for t in &eps[0] {
        if t.from == 0 {
            nfa.add_state(t.from, false);
        }
        nfa.add_state(t.to, false);
        nfa.transitions.push(t.clone());

        println!("{} --> {}", t.from, t.to);
    }
    println!("RIGHT");
    for t in &eps[1] {
        nfa.add_state(t.to, false);
        nfa.transitions.push(t.clone());
        println!("{} --> {}", t.from, t.to);
    }

    nfa.display();
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
                Kind::Or => {
                    if let Some(mut r) = nfas.pop() {
                        if let Some(mut l) = nfas.pop() {
                            nfas.push(make_union_between(&mut l, &mut r));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if let Some(nfa) = nfas.last() {
        //nfa.display();
        nfa.proto_display();
        println!("DEBUG => {:?}", nfas);
    }
    todo!()
}
