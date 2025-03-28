use std::{env, process::exit};

use scanner::parsing::structure::ScanParser;

mod scanner;

fn main() {
    let file = env::args().nth(1).unwrap_or_else(|| {
        println!("need 1 argument !");
        exit(1);
    });

    ScanParser::parse(&file);

    println!("{file}");
}
