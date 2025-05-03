use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

mod srec;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let cli_args = CliArgs::new(&args);
    let outfile = cli_args.outfile.to_string();
    let filename = cli_args.infile.to_string();
    let mode = cli_args.mode.to_string();
    if mode == "uboot" {
        match parse_file(&filename, &outfile) {
            Ok(()) => {
                println!("Successfully extracted hex code from '{}' and wrote to '{}'", filename, outfile);
            }
            Err(e) => {
                eprint!("Error processing file: {}", e);
            }
        }
        Ok(())
    } else if mode == "srec" {
        match srec::parse_srecord_file(&filename, &outfile) {
            Ok(()) => {
                // Process the records if needed
                println!("Successfully extracted hex code from '{}' and wrote to '{}'", filename, outfile);
            }
            Err(e) => {
                eprint!("Error processing file: {}", e);
            }
        }
        Ok(())
    } else {
        eprintln!("Unsupported mode: {}", mode);
        Ok(())
    }
}

struct CliArgs {
    mode: String,
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
        let mode = if args.len() > 3 {
            args[3].clone()
        } else {
            "uboot".to_string()
        };
        if mode != "uboot" && mode != "srec" {
            panic!("Invalid mode: {}. Use 'uboot' or 'srec'.", mode);
        }

        CliArgs { infile, outfile, mode }
    }
}

fn parse_file(path: &str, outfile: &str) -> io::Result<()> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);  
    let mut output_file = fs::File::create(outfile)?;
    let mut hexstr = String::new();

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let line = parts[1].trim().to_string();
                    let line = remove_space(&line)[..32].to_string();
                    for char in line.chars() {
                        if char.is_ascii_hexdigit() {
                            hexstr.push(char);
                        }
                    }
                    for i in (0..hexstr.len()).step_by(2) {
                        if i + 1 < hexstr.len() {
                            let byte_str = &hexstr[i..i + 2];
                            if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
                                output_file.write_all(&[byte])?;
                            } else {
                                eprintln!("Warning: Invalid hex sequence '{}' in line: {}", byte_str, line);
                            }
                        } else if hexstr.len() % 2 != 0 {
                            eprintln!("Warning: Odd number of hex digits at the end of a line: {}", line);
                        }
                    }
                    hexstr.clear(); // Clear hexstr for the next line
                } else {
                    eprintln!("Warning: Invalid line format: {}, skipping...", line);
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }
    Ok(())
}

fn remove_space(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}