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
    pub fn proto_display(&self) {
        println!("==================== NFA ====================");
        println!("States:");

        let mut sorted_states: Vec<&State> = self.states.values().collect();
        sorted_states.sort_by_key(|s| s.id);

        for state in &sorted_states {
            let final_marker = if state.is_final { " (Final)" } else { "" };
            println!("  [{}]{}", state.id, final_marker);
        }

        println!("\nTransitions:");

        let mut transition_map: HashMap<usize, Vec<(Option<char>, usize)>> = HashMap::new();

        for transition in &self.transitions {
            let symbol_str = match &transition.symbol {
                Some(symbol) => Some(symbol.content.clone()),
                None => None,
            };
            transition_map
                .entry(transition.from)
                .or_insert_with(Vec::new)
                .push((symbol_str, transition.to));
        }

        // Afficher les transitions organisées
        for (state_id, transitions) in &transition_map {
            print!("  [{}] → ", state_id);

            let mut transition_strings = Vec::new();
            for (symbol, target) in transitions {
                let transition_str = match symbol {
                    Some(s) => format!("({}) → [{}]", s, target),
                    None => format!("(ε) → [{}]", target),
                };
                transition_strings.push(transition_str);
            }

            println!("{}", transition_strings.join(" | "));
        }

        println!("=============================================");
    }
}
