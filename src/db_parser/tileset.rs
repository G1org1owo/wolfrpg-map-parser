use crate::byte_utils::{as_u32_le, as_u32_vec, parse_string};
use crate::db_parser::tile::Tile;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq)]
pub struct Tileset {
    index: usize,
    name: String,
    base_tileset: String,
    auto_tiles: [String; 15],
    tiles: Vec<Tile>
}

impl Tileset {
    pub(crate) fn parse(bytes: &[u8], index: usize) -> (usize, Self) {
        let mut offset: usize = 0;

        let (bytes_read, name): (usize, String) = parse_string(&bytes[offset..]);
        offset += bytes_read;
        
        let (bytes_read, base_tileset): (usize, String) = parse_string(&bytes[offset..]);
        offset += bytes_read;
        
        let mut auto_tiles: Vec<String> = Vec::with_capacity(15);
        
        for _ in 0..15 {
            let (bytes_read, string): (usize, String) = parse_string(&bytes[offset..]);
            offset += bytes_read;
            auto_tiles.push(string);
        }
        
        let auto_tiles: [String; 15] = auto_tiles.try_into().unwrap();
        
        offset += 1; // Padding
        
        let tags_len: usize = as_u32_le(&bytes[offset..]) as usize;
        offset += 4;
        
        let tag_numbers: Vec<u8> = bytes[offset..][..tags_len].to_vec();
        offset += tags_len;
        
        offset += 1; // Padding
        
        // Should be equal to tags_len
        let directions_len: usize = as_u32_le(&bytes[offset..]) as usize; 
        offset += 4;
        
        let directions: Vec<u32> = as_u32_vec(&bytes[offset..][..4 * directions_len]);
        offset += 4 * directions_len;
        
        let tiles: Vec<Tile> = tag_numbers.iter()
            .zip(directions.iter())
            .map(|(tag, options)| Tile::new(*tag, *options))
            .collect();
        
        (offset, Self {
            index,
            name,
            base_tileset,
            auto_tiles,
            tiles
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

    pub fn base_tileset(&self) -> &str {
        &self.base_tileset
    }
    
    pub fn base_tileset_mut(&mut self) -> &mut String {
        &mut self.base_tileset
    }

    pub fn auto_tiles(&self) -> &[String; 15] {
        &self.auto_tiles
    }
    
    pub fn auto_tiles_mut(&mut self) -> &mut [String; 15] {
        &mut self.auto_tiles
    }

    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
    
    pub fn tiles_mut(&mut self) -> &mut Vec<Tile> {
        &mut self.tiles
    }
}