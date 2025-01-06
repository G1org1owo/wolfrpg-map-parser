use crate::byte_utils::{as_u32_be, as_u32_le, parse_string};
use crate::page::Page;
#[cfg(feature = "serde")]
use serde::Serialize;

const EVENT_SIGNATURE: u32 = 0x6f393000;

#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Event {
    id: u32,
    name: String,
    position_x: u32,
    position_y: u32,
    unknown1: u32,
    pages: Vec<Page>
}

impl Event {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let signature: u32 = as_u32_be(&bytes[offset..offset + 4]);
        offset += 4;

        if signature != EVENT_SIGNATURE {
            panic!("Invalid event signature: {:02x}.", signature);
        }

        offset += 1; // padding

        let id: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let (bytes_read, name): (usize, String) = parse_string(&bytes[offset..]);
        offset += bytes_read;

        let position_x: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let position_y: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let page_count: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let unknown1: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let mut pages: Vec<Page> = vec![];
        for _ in 0..page_count {
            let (bytes_read, page): (usize, Page) = Page::parse(&bytes[offset..]);
            offset += bytes_read;
            pages.push(page);
        }

        let event_end: u8 = bytes[offset];
        offset += 1;

        if event_end != 0x70 {
            panic!("Expected event end but found {:02x}.", event_end);
        }

        (offset, Self {
            id,
            name,
            position_x,
            position_y,
            unknown1,
            pages,
        })
    }

    pub fn parse_multiple(bytes: &[u8], count: u32) -> (usize, Vec<Self>) {
        let mut offset: usize = 0;
        let mut events: Vec<Event> = Vec::new();

        for _ in 0..count {
            let (bytes_read, event): (usize, Self) = Self::parse(&bytes[offset..]);

            offset += bytes_read;
            events.push(event);
        }

        (offset, events)
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn position_x(&self) -> u32 {
        self.position_x
    }

    pub fn position_x_mut(&mut self) -> &mut u32 {
        &mut self.position_x
    }

    pub fn position_y(&self) -> u32 {
        self.position_y
    }

    pub fn position_y_mut(&mut self) -> &mut u32 {
        &mut self.position_y
    }

    pub fn pages(&self) -> &Vec<Page> {
        &self.pages
    }

    pub fn pages_mut(&mut self) -> &mut Vec<Page> {
        &mut self.pages
    }
}