

pub struct SRecord {
    pub record_type: u8,  // Record type (e.g., S0, S1, S2, etc.)
    pub address: u32,    // Address field
    pub data: Vec<u8>,   // Data bytes
    pub checksum: u8,    // Checksum byte
}

impl SRecord {
    pub fn parse(line: &str) -> Result<SRecord, String> {
        if line.len() < 10 || !line.starts_with('S') {
            return Err("Invalid SRecord line".to_string());
        }

        let record_type = match line.chars().nth(1) {
            Some(c) if c.is_digit(10) => c.to_digit(10).unwrap() as u8,
            _ => return Err("Invalid record type".to_string()),
        };

        let byte_count = u8::from_str_radix(&line[2..4], 16)
            .map_err(|_| "Invalid byte count".to_string())?;

        let address_length = match record_type {
            0 | 1 | 5 | 9 => 4,
            2 | 8 => 6,
            3 | 7 => 8,
            _ => return Err("Unsupported record type".to_string()),
        };

        if line.len() < 4 + address_length + 2 {
            return Err("Line too short for address and checksum".to_string());
        }

        let address = u32::from_str_radix(&line[4..4 + address_length], 16)
            .map_err(|_| "Invalid address".to_string())?;

        let data_start = 4 + address_length;
        let data_end = line.len() - 2;

        let data = (data_start..data_end)
            .step_by(2)
            .map(|i| u8::from_str_radix(&line[i..i + 2], 16))
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_| "Invalid data bytes".to_string())?;

        let checksum = u8::from_str_radix(&line[data_end..], 16)
            .map_err(|_| "Invalid checksum".to_string())?;

        let calculated_checksum = byte_count
            .wrapping_add((address >> 24) as u8)
            .wrapping_add((address >> 16) as u8)
            .wrapping_add((address >> 8) as u8)
            .wrapping_add(address as u8)
            .wrapping_add(data.iter().copied().sum::<u8>())
            .wrapping_neg();

        if checksum != calculated_checksum {
            return Err("Checksum mismatch".to_string());
        }

        Ok(SRecord {
            record_type,
            address,
            data,
            checksum,
        })
    }
}