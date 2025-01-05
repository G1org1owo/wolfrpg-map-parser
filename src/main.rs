pub mod map;
pub mod event;
pub mod command;
mod byte_utils;
pub mod common;
pub mod page;

use std::{env, fs};
use crate::map::Map;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let bytes: Vec<u8> = fs::read(&args[1]).expect("Error reading file!");
    let map: Map = Map::parse(&bytes);

    println!("{}", serde_json::to_string_pretty(&map).expect("Serialization failed!"));
    fs::write("out.json", serde_json::to_string_pretty(&map).expect("Serialization failed!"))
        .expect("Write failed!");
}