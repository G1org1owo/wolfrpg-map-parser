use crate::byte_utils::{as_blob, as_u32_le, as_u32_vec, parse_string, parse_string_vec};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq)]
pub struct TypeInfo {
    index: usize,
    name: String,
    fields: Vec<TypeField>,
    data_names: Vec<String>,
    note: String,
}

impl TypeInfo {
    pub(crate) fn parse(bytes: &[u8], index: usize) -> (usize, Self) {
        let mut offset: usize = 0;
        
        let (bytes_read, name): (usize, String) = parse_string(&bytes[offset..]);
        offset += bytes_read;

        let field_count: usize = as_u32_le(&bytes[offset..]) as usize;
        offset += 4;

        let (bytes_read, fields): (usize, Vec<String>)
            = parse_string_vec(&bytes[offset..], field_count);
        offset += bytes_read;

        let data_count: usize = as_u32_le(&bytes[offset..]) as usize;
        offset += 4;

        let (bytes_read, data_names): (usize, Vec<String>)
            = parse_string_vec(&bytes[offset..], data_count);
        offset += bytes_read;

        let (bytes_read, note): (usize, String) = parse_string(&bytes[offset..]);
        offset += bytes_read;

        let (bytes_read, field_data_types): (usize, Vec<u8>)
            = as_blob(&bytes[offset..], 1);
        offset += bytes_read;

        let (bytes_read, _) = as_blob(&bytes[offset..], 5);
        offset += bytes_read;

        let _ = as_u32_le(&bytes[offset..]);
        offset += 4;

        let mut field_strings: Vec<Vec<String>> = Vec::with_capacity(field_count);

        for _ in 0..field_count {
            let string_count: usize = as_u32_le(&bytes[offset..]) as usize;
            offset += 4;

            let (bytes_read, strings): (usize, Vec<String>)
                = parse_string_vec(&bytes[offset..], string_count);
            offset += bytes_read;

            field_strings.push(strings);
        }

        let _ = as_u32_le(&bytes[offset..]);
        offset += 4;

        let mut field_metas: Vec<Vec<i32>> = Vec::with_capacity(field_count);

        for _ in 0..field_count {
            let count: usize = as_u32_le(&bytes[offset..]) as usize;
            offset += 4;

            let field_meta: Vec<i32> = as_u32_vec(&bytes[offset..][..4 * count])
                .iter().map(|u| *u as i32)
                .collect();
            offset += 4 * count;

            field_metas.push(field_meta);
        }

        let _ = as_u32_le(&bytes[offset..]);
        offset += 4;

        let default_values: Vec<i32> = as_u32_vec(&bytes[offset..][.. 4 * field_count])
            .iter().map(|u| *u as i32)
            .collect();
        offset += 4 * field_count;

        let fields: Vec<TypeField> = fields.iter().enumerate().map(|(i, str)| {
            TypeField {
                name: str.clone(),
                special: field_data_types[i],
                strings: field_strings[i].clone(),
                meta: field_metas[i].clone(),
                default: default_values[i]
            }
        }).collect();

        (offset, Self {
            index,
            name,
            fields,
            data_names,
            note
        })
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn fields(&self) -> &Vec<TypeField> {
        &self.fields
    }
    
    pub fn fields_mut(&mut self) -> &mut Vec<TypeField> {
        &mut self.fields
    }

    pub fn data_names(&self) -> &Vec<String> {
        &self.data_names
    }
    
    pub fn data_names_mut(&mut self) -> &mut Vec<String> {
        &mut self.data_names
    }

    pub fn note(&self) -> &str {
        &self.note
    }
    
    pub fn note_mut(&mut self) -> &mut String {
        &mut self.note
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq)]
pub struct TypeField {
    name: String,
    special: u8,
    strings: Vec<String>,
    meta: Vec<i32>,
    default: i32
}

impl TypeField {
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn special(&self) -> u8 {
        self.special
    }
    
    pub fn special_mut(&mut self) -> &mut u8 {
        &mut self.special
    }

    pub fn strings(&self) -> &Vec<String> {
        &self.strings
    }
    
    pub fn strings_mut(&mut self) -> &mut Vec<String> {
        &mut self.strings
    }

    pub fn meta(&self) -> &Vec<i32> {
        &self.meta
    }
    
    pub fn meta_mut(&mut self) -> &mut Vec<i32> {
        &mut self.meta
    }

    pub fn default(&self) -> i32 {
        self.default
    }
    
    pub fn default_mut(&mut self) -> &mut i32 {
        &mut self.default
    }
}