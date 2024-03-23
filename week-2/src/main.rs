use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};

fn read_file(file_path: &str) {
    let file = File::open(file_path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    let reader = BufReader::new(file);
    println!("===== Dumping file contents for {} ======", file_path);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("The executable's path is {}", args[0]);
    if args.len() < 2 {
        panic!("Please provide a file path as an argument");
    }
    read_file(&args[1]);
    
}