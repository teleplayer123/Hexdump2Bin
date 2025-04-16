use std::env;
use std::fs;



fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1].to_string();
    let data = parse_file(filename);
    println!("Contents: {}", &data);
}

fn parse_file(path: &String) -> String {
    println!("Opening file: {}", path);
    let contents = fs::read_to_string(path).expect("Error reading file");    
    contents
}
