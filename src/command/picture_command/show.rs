use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::picture_command::options::Options;
use state::State;

mod state;
mod base;
mod free_transform;
mod delay;
mod parser;
mod zoom;
mod color_values;
mod range_state;
mod color_values_state;
mod zoom_state;
mod free_transform_state;
mod delay_state;
mod range;
mod colors_state;
mod colors;

#[derive(Serialize)]
pub struct Show {
    options: Options,
    picture: u32,
    process_time: u32,
    division_width: u32,
    division_height: u32,
    pattern: u32,
    opacity: u32,
    zoom: u32,
    angle: u32,
    state: State
}

impl Show {
    fn parse(bytes: &[u8], parse_state: fn(&[u8], &Options) -> (usize, State)) -> (usize, Self) {
        let mut offset: usize = 0;

        let options: u32 = as_u32_le(&bytes[offset..offset+4]);
        let options: Options = Options::new(options);
        offset += 4;

        let picture: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let process_time: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let division_width: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let division_height: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let pattern: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let opacity: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let zoom: u32 = as_u32_le(&bytes[offset+8..offset+12]);
        let angle: u32 = as_u32_le(&bytes[offset+12..offset+16]);

        let (bytes_read, state): (usize, State) = parse_state(&bytes[offset..], &options);
        offset += bytes_read;

        offset += 1; // Command end signature

        (offset, Self {
            options,
            picture,
            process_time,
            division_width,
            division_height,
            pattern,
            opacity,
            zoom,
            angle,
            state
        })
    }

    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_base)
    }

    pub fn parse_colors(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_colors)
    }

    pub fn parse_delay(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_delay)
    }

    pub fn parse_range(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_range)
    }

    pub fn parse_color_values(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_color_values)
    }

    pub fn parse_zoom(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_zoom)
    }

    pub fn parse_free_transform(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_free_transform)
    }
}