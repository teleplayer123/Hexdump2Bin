use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

fn hex_dump_to_bin(input_path: &PathBuf, output_path: &PathBuf) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    let mut output_file = File::create(output_path)?;

    for line_result in reader.lines() {
        let line = line_result?;
        let mut hex_string = String::new();

        // Iterate through characters and keep only valid hex digits
        for char in line.chars() {
            if char.is_ascii_hexdigit() {
                hex_string.push(char);
            }
        }

        // Process the hex string in pairs of characters (bytes)
        for i in (0..hex_string.len()).step_by(2) {
            if i + 1 < hex_string.len() {
                let byte_str = &hex_string[i..i + 2];
                if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
                    output_file.write_all(&[byte])?;
                } else {
                    eprintln!("Warning: Invalid hex sequence '{}' in line: {}", byte_str, line);
                }
            } else if hex_string.len() % 2 != 0 {
                eprintln!("Warning: Odd number of hex digits at the end of a line: {}", line);
                // You might want to handle this differently, e.g., pad with a zero or ignore
            }
        }
    }

    println!(
        "Successfully extracted hex code from '{}' and wrote to '{}'",
        input_path.display(),
        output_path.display()
    );

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: cargo run <input_hex_dump_file> <output_binary_file>");
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);
    let output_path = PathBuf::from(&args[2]);

    hex_dump_to_bin(&input_path, &output_path)?;

    Ok(())
}
