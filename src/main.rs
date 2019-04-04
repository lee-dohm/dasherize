extern crate regex;

use regex::Regex;
use std::env;

fn main() {
    let mut text: String = get_arg_text();
    text = dasherize(text);

    println!("{}", text);
}

/// Returns the dasherized version of the given text.
fn dasherize(mut text: String) -> String {
    let chars_pattern = Regex::new(r"[^a-z0-9]+").unwrap();
    text = text.trim().to_lowercase();

    chars_pattern
        .replace_all(&text, "-")
        .trim_end_matches("-")
        .trim_start_matches("-")
        .to_string()
}

/// Gets the text passed as arguments on the command line, returning it as one string.
fn get_arg_text() -> String {
    let args: Vec<String> = env::args().collect();

    args[1..].join(" ")
}
