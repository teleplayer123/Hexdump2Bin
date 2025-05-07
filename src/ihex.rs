use std::fs;
use std::io::{self, BufRead, Write};

//parse intel hex file

struct IHexRecord {
    length: u8,
    address: u16,
    record_type: u8,
    data: Vec<u8>,
    checksum: u8,
}

