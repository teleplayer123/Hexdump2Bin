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

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_and_process_log(filepath: &str) -> io::Result<Vec<String>> {
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);
    let mut results = Vec::new();

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    results.push(parts[1].trim().to_string());
                } else {
                    eprintln!("Warning: Skipping line due to invalid format (no ':' found): {}", line);
                    // Optionally, you could log this to a separate error file
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                // Optionally, you could handle the error differently, e.g., stop processing
            }
        }
    }

    Ok(results)
}

fn main() -> io::Result<()> {
    let log_file_path = "example.log"; // Replace with your log file path

    // Create a dummy log file for testing
    std::fs::write(
        log_file_path,
        "INFO: Application started\nDEBUG: Processing request 123\nERROR: Failed to connect to database\nWARNING: Low disk space\nInvalid line without colon\nDATA: Some important data here\nANOTHER: More data",
    )?;

    println!("Reading and processing log file: {}", log_file_path);

    match read_and_process_log(log_file_path) {
        Ok(extracted_parts) => {
            println!("\nExtracted parts after splitting by ':'");
            for part in extracted_parts {
                println!("{}", part);
            }
        }
        Err(e) => {
            eprintln!("Error processing log file: {}", e);
        }
    }

    Ok(())
}
