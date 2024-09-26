mod map;
mod event;
mod command;
mod byte_utils;
mod show_message_command;
mod show_choice_command;
mod case;

use std::{env, fs};
use serde::{Serialize};
use serde_json;
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
}