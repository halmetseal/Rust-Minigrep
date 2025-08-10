use std::{env, process};

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

}
