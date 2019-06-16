use std::env;
use std::fs;
use std::process;
use std::io;

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
    println!("{}", contents);
}
