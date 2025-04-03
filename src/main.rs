use std::{env, process::exit};

use scanner::parsing::tokenizer::structure::ScanParser;

mod scanner;

fn main() {
    let file = env::args().nth(1).unwrap_or_else(|| {
        println!("need 1 argument !");
        exit(1);
    });
    let mut parser = ScanParser::new();

    let mut bef = parser.parse(&file);
    println!(
        "-------------------------------[BEFORE]-------------------------------\n{}",
        bef
    );
    bef.to_postfix();
    println!(
        "-------------------------------[AFTER]-------------------------------\n{}",
        bef
    );
    println!("{file}");
}
