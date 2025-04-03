use std::collections::HashMap;

use crate::scanner::parsing::tokenizer::structure::Token;

#[derive(Debug, Clone)]
pub struct State {
    pub id: usize,
    pub is_final: bool,
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub from: usize,
    pub to: usize,
    pub symbol: Option<Token>,
}

#[derive(Debug, Clone)]
pub struct NFA {
    pub states: HashMap<usize, State>,
    pub transitions: Vec<Transition>,
}

impl NFA {
    pub fn new() -> Self {
        NFA {
            states: HashMap::new(),
            transitions: Vec::new(),
        }
    }

    pub fn add_state(&mut self, id: usize, is_final: bool) {
        let state = State { id, is_final };
        self.states.insert(id, state);
    }
    pub fn add_transition(&mut self, from: usize, to: usize, symbol: Option<Token>) {
        let transition = Transition { from, to, symbol };
        self.transitions.push(transition);
    }

    pub fn display(&self) {
        println!("States:");
        for state in self.states.values() {
            println!("State ID: {}, Final: {}", state.id, state.is_final);
        }
        println!("\nTransitions:");
        let mut last: usize = 4000;
        for transition in &self.transitions {
            match &transition.symbol {
                Some(symbol) => {
                    if last != transition.from {
                        print!(
                            "{} --{}--> {}",
                            transition.from, symbol.content, transition.to
                        )
                    } else {
                        print!(" --{}--> {}", symbol.content, transition.to)
                    }
                }
                None => println!("{} --ε--> {}", transition.from, transition.to),
            }
            last = transition.to;
        }
        println!("");
    }
}
