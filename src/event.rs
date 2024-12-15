mod page;

use serde::Serialize;
use crate::byte_utils::{as_string, as_u32_le};
use crate::event::page::Page;

#[derive(Serialize)]
pub struct Event {
    id: u32,
    name: String,
    position_x: u32,
    position_y: u32,
    page_count: u32,
    unknown1: u32,
    pages: Vec<Page>
}

impl Event {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 5;
        let id: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let name_length: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset+=4;

        let name: String = as_string(bytes, offset, name_length);
        offset+=name_length;

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
        offset+=1; // TODO: throw error if not page/event end signature

        (offset, Self {
            id,
            name,
            position_x,
            position_y,
            page_count,
            unknown1,
            pages,
        })
    }
}