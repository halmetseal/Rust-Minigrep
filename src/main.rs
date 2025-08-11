use std::{
    env::{self},
    fs, process,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(&config.file_path)
        .unwrap_or_else(|_| panic!("Could not read the specified file {}", config.file_path));

    println!("File contents: ");

    println!("{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let binary_name = &args[0];

        if args.len() < 3 {
            print!("Usage: {binary_name} <searchstring> <file_path>");
            process::exit(1)
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
