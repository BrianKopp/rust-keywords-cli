use std::env;
use std::fs;
use std::process;
use std::io;
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
    for content_match in re.find_iter(&contents) {
        println!("{}", content_match.as_str());
    }
}
