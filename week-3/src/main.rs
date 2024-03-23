use std::env;
#[derive(Debug)]
struct FileSize {    
    size: u64,
}
impl std::fmt::Display for FileSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let size = self.size;
        write!(f, "bytes: {}, KB: {}, MB: {}, GB: {}", size, size/1_000, size/1_000_000, size/1_000_000_000)
    }
}

    fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a size string as an argument, like 'n mb' or 'n gb'");
    }

    let s: Vec<&str> = args[1].split_whitespace().collect();
        
    let size_in_bytes = match s[1] {
        "b" | "B" => s[0].parse().expect("Invalid number"),
        "kb" | "KB" => s[0].parse::<u64>().expect("Invalid number")*1_000,
        "mb" | "MB" => s[0].parse::<u64>().expect("Invalid number")*1_000_000,
        "gb" | "GB" => s[0].parse::<u64>().expect("Invalid number")*1_000_000_000,
        _ => panic!("Invalid size specifier. Use 'b', 'kb', 'mb', or 'gb'"),
    };
    let s = FileSize { size: size_in_bytes };
    println!("{}", s);
}