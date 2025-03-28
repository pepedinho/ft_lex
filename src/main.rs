use std::{env, fs, process::exit};

use scanner::parsing::structure::ScanParser;

mod scanner;

fn main() {
    let file = env::args().nth(1).unwrap_or_else(|| {
        println!("need 1 argument !");
        exit(1);
    });

    ScanParser::parse(&file);
    //let contents = fs::read_to_string(&file).expect("Should have been able to read the file");

    println!("{file}");
    //println!("contents : \n{contents}");
}
