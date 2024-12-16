use serde::Serialize;
use crate::byte_utils::{as_u32_vec, as_u32_le};
use crate::event::Event;

const MAP_SIGNATURE: &[u8]
    = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x57\x4F\x4C\x46\x4D\x00\x00\x00\x00\x00";

#[derive(Serialize)]
pub struct Map {
    tileset: u32,
    width: u32,
    height: u32,
    layer1: Vec<u32>,
    layer2: Vec<u32>,
    layer3: Vec<u32>,
    events: Vec<Event>,
}

impl Map {
    pub fn parse(bytes: &[u8]) -> Self {
        let mut offset: usize = 0;//(0x1D + as_u32_le(&bytes[0x19..0x1D])) as usize;

        let magic: &[u8] = &bytes[offset..offset+20];
        offset += 20;

        if magic != MAP_SIGNATURE {
            panic!("Invalid WOLF map signature.");
        }

        offset += 5; // Unknown data

        let skippable: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset += 4;
        offset += skippable;

        let tileset: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let width: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let height: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let event_count: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let layer_length: usize = (width * height * 4) as usize;
        let layer1: Vec<u32> = as_u32_vec(
            &bytes[offset..offset + layer_length],
        );
        offset += layer_length;

        let layer2: Vec<u32> = as_u32_vec(
            &bytes[offset..offset + layer_length]
        );
        offset += layer_length;

        let layer3: Vec<u32> = as_u32_vec(
            &bytes[offset..offset + layer_length]
        );
        offset += layer_length;

        let (bytes_read, events): (usize, Vec<Event>)
            = Event::parse_multiple(&bytes[offset..], event_count);
        offset += bytes_read;

        let map_end: u8 = bytes[offset];

        if map_end != 0x66 {
            panic!("Expected map end but found {:02x}.", map_end);
        }

        Self {
            tileset,
            width,
            height,
            layer1,
            layer2,
            layer3,
            events
        }
    }

    pub fn tileset(&self) -> u32 {
        self.tileset
    }

    pub fn tileset_mut(&mut self) -> &mut u32 {
        &mut self.tileset
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn width_mut(&mut self) -> &mut u32 {
        &mut self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn height_mut(&mut self) -> &mut u32 {
        &mut self.height
    }

    pub fn layer1(&self) -> &Vec<u32> {
        &self.layer1
    }

    pub fn layer1_mut(&mut self) -> &mut Vec<u32> {
        &mut self.layer1
    }

    pub fn layer2(&self) -> &Vec<u32> {
        &self.layer2
    }

    pub fn layer2_mut(&mut self) -> &mut Vec<u32> {
        &mut self.layer2
    }

    pub fn layer3(&self) -> &Vec<u32> {
        &self.layer3
    }

    pub fn layer3_mut(&mut self) -> &mut Vec<u32> {
        &mut self.layer3
    }

    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn events_mut(&mut self) -> &mut Vec<Event> {
        &mut self.events
    }
}