use crate::byte_utils::as_u32_le;
use crate::db_parser::table::Table;
use std::fs;
use std::io::Result;
use std::path::Path;
use crate::db_parser::DATA_MAGIC;

pub fn parse(data: &Path) -> Result<Vec<Table>> {
    match fs::read(data) {
        Ok(contents) => {
            Ok(parse_bytes(&contents))
        }
        Err(e) => {
            Err(e)
        }
    }
}

#[allow(unused_assignments)]
pub fn parse_bytes(bytes: &[u8]) -> Vec<Table> {
    let mut offset: usize = 0;
    
    let header: &[u8] = &bytes[0..11];
    offset += 11;

    if &header[..10] != DATA_MAGIC {
        panic!("Invalid data header.");
    }
    
    let type_count: usize = as_u32_le(&bytes[offset..]) as usize;
    offset += 4;
    
    let mut tables = Vec::with_capacity(type_count);
    
    for i in 0..type_count {
        let (bytes_read, table): (usize, Table) = Table::parse(&bytes[offset..], i);
        offset += bytes_read;
        tables.push(table);
    }
    
    offset += 1; // Should be 0xc1
    
    tables
}