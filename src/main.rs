extern crate regex;

use std::env;
use regex::Regex;

fn main() {
    let chars_pattern = Regex::new(r"[^a-z0-9]+").unwrap();
    let args: Vec<String> = env::args().collect();
    let mut text: String = args[1..].join(" ").trim().to_lowercase();
    text =
        chars_pattern
        .replace_all(&text, "-")
        .trim_end_matches("-")
        .trim_start_matches("-")
        .to_string();

    println!("{}", text);
}
