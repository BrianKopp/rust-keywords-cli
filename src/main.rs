use std::env;
use std::fs;
use std::process;
use std::io;
use std::collections::HashMap;
extern crate regex;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];
    let file_content_result = fs::read_to_string(file_name);
    if let Err(err) = file_content_result {
        match err.kind() {
            io::ErrorKind::NotFound => println!("file '{}' not found", file_name),
            _ => println!("unexpected error occurred: {}", err),
        }
        process::exit(1)
    }
    let contents = file_content_result.unwrap();

    // create regular expression
    let re = Regex::new(r"[a-zA-Z_$][a-zA-Z_$0-9]*").unwrap();
    let mut counts_by_word: HashMap<&str, u32> = HashMap::new();
    for content_match in re.find_iter(&contents) {
        let count = counts_by_word
            .entry(content_match.as_str())
            .or_insert(0);
        *count += 1;
    }

    println!("Results:");
    let mut sorted_words: Vec<_> = counts_by_word.iter().collect();
    sorted_words.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    for result in sorted_words.iter() {
        println!("{} - {}", result.0, result.1);
    }
}