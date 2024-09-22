use std::{env, fs};
use std::io::Read;
use serde::{Serialize};
use serde_json;
#[derive(Serialize)]
struct Map {
    tileset: u32,
    width: u32,
    height: u32,
    layer1: Vec<u32>,
    layer2: Vec<u32>,
    layer3: Vec<u32>
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let bytes: Vec<u8> = fs::read(&args[1]).expect("Error reading file!");
    let map: Map = parse_map(&bytes);

    println!("{}", serde_json::to_string_pretty(&map).expect("Serialization failed!"));
}

fn parse_map(bytes: &[u8]) -> Map {
    let offset: usize = (0x1D + as_u32_le(&bytes[0x19..0x1D])) as usize;

    let tileset: u32 = as_u32_le(&bytes[offset..offset+4]);
    let width: u32 = as_u32_le(&bytes[offset+4..offset+8]);
    let height: u32 = as_u32_le(&bytes[offset+8..offset+12]);

    let layer1 = as_u32_array(
        &bytes[offset + 12usize..offset + 12usize + (width * height) as usize]
    ).to_vec();

    let layer2 = as_u32_array(&bytes[
        offset+12usize + (width*height) as usize..offset+12usize + (width*height*2) as usize
    ]).to_vec();
    let layer3 = as_u32_array(&bytes[
        offset+12usize + (width*height*2) as usize..offset+12usize + (width*height*3) as usize
    ]).to_vec();

    Map {
        tileset,
        width,
        height,
        layer1,
        layer2,
        layer3,
    }
}

fn as_u32_le(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32) <<  0) |
    ((bytes[1] as u32) <<  8) |
    ((bytes[2] as u32) << 16) |
    ((bytes[3] as u32) << 24)
}

fn as_u32_be(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32) << 24) |
    ((bytes[1] as u32) << 16) |
    ((bytes[2] as u32) <<  8) |
    ((bytes[3] as u32) <<  0)
}

fn as_u32_array(bytes: &[u8]) -> &[u32] {
    unsafe {
        let (_, data, _): (&[u8], &[u32], &[u8]) = bytes.align_to::<u32>();

        data
    }
}