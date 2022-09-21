use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With the text: \n{contents}")
}
struct Config {
    _query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments.");
        }
        Ok(Config {
            _query: args[1].clone(),
            file_path: args[2].clone()
        })
    }
}
