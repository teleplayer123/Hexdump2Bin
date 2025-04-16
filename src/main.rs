use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
// use std::{fmt, num::ParseIntError};


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let cli_args = CliArgs::new(&args);
    // let mut outfile = fs::File::create(cli_args.outfile.to_string())?;
    let outfile = cli_args.outfile.to_string();
    let filename = cli_args.infile.to_string();
    let data = parse_file(&filename)?;
    let bytes = hex_to_bytes(&data.join(" "));
    match bytes {
        Ok(bytes) => {
            write_to_binary_file(&bytes, &outfile);
        }
        Err(_) => {
            eprint!("Error converting hex to bytes");
        }
    }
    // match parse_file(&filename) {
    //     Ok(parts) => {
    //         for part in parts {
    //             write!(outfile, "{}", part).expect("Error writing to log file");
    //         }
    //     }
    //     Err(e) => {
    //         eprint!("Error processing file: {}", e);
    //     }
    // }
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
                let parts: Vec<&str> = line.splitn(2, ":").collect();
                if parts.len() == 2 {
                    results.push(parts[1].trim().to_string());
                } else {
                    eprint!("Warning: skipping invalid line\n");
                }
            }
            Err(e) => {
                eprint!("Error reading line: {}\n", e);
            }
        }
    }
    Ok(results)
}

fn hex_to_bytes(hex_string: &str) -> Result<Vec<u8>, ()> {
    let mut bytes = Vec::new();
    for i in (0..hex_string.len()).step_by(2) {
        let c1 = hex_string.chars().nth(i).unwrap();
        let c2 = hex_string.chars().nth(i + 1).unwrap();
        let byte: u8 = match (c1, c2) {
            ('0'..='9', 'a'..='f') | ('a'..='f', '0'..='9') => {
                u8::from_str_radix(&format!("{}{}", c1, c2), 16)
                    .map_err(|_| ())?
            }
            _ => return Err(()),
        };
        bytes.push(byte);
    }
    Ok(bytes)
}

fn write_to_binary_file(bytes: &[u8], outfile: &str) {
    let mut file = match fs::File::create(outfile) {
        Err(why) => panic!("Couldn't create {}: {}", outfile, why),
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(&bytes[..]) {
        panic!("Couldn't write to {}: {}", "output.bin", why);
    }
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// enum DecodeHexError {
//     OddLength,
//     ParseInt(ParseIntError),
// }

// impl From<ParseIntError> for DecodeHexError {
//     fn from(e: ParseIntError) -> Self {
//         DecodeHexError::ParseInt(e)
//     }
// }

// impl fmt::Display for DecodeHexError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             DecodeHexError::OddLength => "input string has an odd number of bytes".fmt(f),
//             DecodeHexError::ParseInt(e) => e.fmt(f),
//         }
//     }
// }

// impl std::error::Error for DecodeHexError {}

// fn decode_hex(s: &str) -> Result<Vec<u8>, DecodeHexError> {
//     if s.len() % 2 != 0 {
//         Err(DecodeHexError::OddLength)
//     } else {
//         (0..s.len())
//             .step_by(2)
//             .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.into()))
//             .collect()
//     }
// }