use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let content = fs::read_to_string(&args[1])
        .expect("should have been able to read file");
    println!("{}", content);
}
