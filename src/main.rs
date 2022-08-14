use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should be able to read a file");

    println!("With text:\n {contents}");
}

impl Config {
    // implement a new function for the config
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
