use std::env;
use std::fs;
use std::io::{self, BufRead};


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let cli_args = CliArgs::new(&args);
    let outfile = cli_args.outfile.to_string();
    let filename = cli_args.infile.to_string();

    match parse_file(&filename) {
        Ok(parts) => {
            for part in parts {
                fs::write(&outfile, part).expect("Error writing to log file");
            }
        }
        Err(e) => {
            eprint!("Error processing file: {}", e);
        }
    }
    Ok(())
}

struct CliArgs {
    infile: String,
    outfile: String,
}

impl CliArgs {
    fn new(args: &[String]) -> CliArgs {
        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        let infile = args[1].clone();
        let outfile = args[2].clone();

        CliArgs { infile, outfile}
    }
}

fn parse_file(path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut results = Vec::new();
    // let contents = fs::read_to_string(path).expect("Error reading file");    
    
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    results.push(parts[1].trim().to_string());
                } else {
                    eprint!("Warning: skipping invalid line");
                }
            }
            Err(e) => {
                eprint!("Error reading line: {}", e);
            }
        }
    }
    Ok(results)
}