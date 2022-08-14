use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should be able to read a file");

    println!("With text:\n {contents}");
}

impl Config {
    // implement a new function for the config
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
