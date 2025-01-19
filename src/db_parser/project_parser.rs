use crate::byte_utils::as_u32_le;
use crate::db_parser::type_info::TypeInfo;
use std::fs;
use std::io::Result;
use std::path::Path;

pub fn parse(project: &Path) -> Result<Vec<TypeInfo>> {
    match fs::read(project) {
        Ok(contents) => {
            Ok(parse_bytes(&contents))
        }
        Err(e) => {
            Err(e)
        }
    }
}



fn parse_bytes(bytes: &[u8]) -> Vec<TypeInfo> {
    let mut offset: usize = 0;
    
    let type_count: usize = as_u32_le(bytes) as usize;
    offset += 4;
    
    let mut types: Vec<TypeInfo> = Vec::with_capacity(type_count);
    
    for i in 0..type_count {
        let (bytes_read, type_info): (usize, TypeInfo) = TypeInfo::parse(&bytes[offset..], i);
        offset += bytes_read;
        types.push(type_info);
    }
    
    types
}