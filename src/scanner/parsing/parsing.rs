use std::fs;

use super::structure::ScanParser;

impl ScanParser {
    pub fn parse(scan_path: &str) -> Self {
        let content =
            fs::read_to_string(&scan_path).expect("Should have been able to read the file");

        println!("content : {content}");

        ScanParser { content }
    }
}
