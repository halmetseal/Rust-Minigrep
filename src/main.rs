use std::{env, fs, process};

fn main() {
    let args: Vec<String>  = env::args().collect();
    // dbg!(args);
    let binary_name = &args[0];

    if args.len()<3 {
        print!("Usage: {binary_name} <searchstring> <file_path>");
        process::exit(1)
    }
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Could not read the specified file {file_path}"));

    println!("File contents: ");

    println!("{contents}");

}
