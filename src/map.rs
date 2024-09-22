use serde::Serialize;
use crate::byte_utils::{as_u32_array, as_u32_le};
use crate::event::Event;

#[derive(Serialize)]
pub struct Map {
    tileset: u32,
    width: u32,
    height: u32,
    event_count: u32,
    layer1: Vec<u32>,
    layer2: Vec<u32>,
    layer3: Vec<u32>,
    events: Vec<Event>,
}

impl Map {
    pub fn parse(bytes: &[u8]) -> Self {
        let mut offset: usize = (0x1D + as_u32_le(&bytes[0x19..0x1D])) as usize;

        let tileset: u32 = as_u32_le(&bytes[offset..offset+4]);
        let width: u32 = as_u32_le(&bytes[offset+4..offset+8]);
        let height: u32 = as_u32_le(&bytes[offset+8..offset+12]);
        let event_count: u32 = as_u32_le(&bytes[offset+12..offset+16]);
        offset += 16;

        let layer1 = as_u32_array(
            &bytes[offset..offset + (width * height * 4) as usize]
        ).to_vec();
        offset += (width * height * 4) as usize;

        let layer2 = as_u32_array(
            &bytes[offset..offset + (width * height * 4) as usize]
        ).to_vec();
        offset += (width * height * 4) as usize;

        let layer3 = as_u32_array(
            &bytes[offset..offset + (width * height * 4) as usize]
        ).to_vec();
        offset += (width * height * 4) as usize;

        let events: Vec<Event> = Map::parse_events(&bytes[offset..], event_count);

        Self {
            tileset,
            width,
            height,
            event_count,
            layer1,
            layer2,
            layer3,
            events
        }
    }

    fn parse_events(bytes: &[u8], event_count: u32) -> Vec<Event> {
        let mut offset: usize = 0;
        let mut events: Vec<Event> = Vec::new();

        for _i in 0..event_count {
            println!("event offset: {offset}");
            let (bytes_read, event): (usize, Event) = Event::parse(&bytes[offset..]);

            offset += bytes_read;
            events.push(event);
        }

        events
    }
}