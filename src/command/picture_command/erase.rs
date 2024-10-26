mod state;
mod delay_reset;
mod base;
mod delay;
mod range;

use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::picture_command::display_type::DisplayType;
use crate::command::picture_command::erase::state::State;
use crate::command::picture_command::options::Options;

#[derive(Serialize)]
pub struct Erase {
    options: Options,
    picture: u32,
    state: State,
}

impl Erase {
    fn parse(bytes: &[u8], parse_state: fn(&[u8], &Options) -> (usize, State)) -> (usize, Self) {
        let mut offset: usize = 0;

        let options: u32 = as_u32_le(&bytes[offset..offset + 4]);
        let options: Options = Options::new(options);
        offset += 4;

        let picture: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let (bytes_read, state) = parse_state(&bytes[offset..], &options);
        offset += bytes_read;

        offset += 3; // Command end signature

        (offset, Self {
            options,
            picture,
            state
        })
    }

    pub fn parse_delay_reset(bytes: &[u8]) -> (usize, Self) {
        let parse_state = |bytes: &[u8], _: &Options| -> (usize, State) {
            State::parse_delay_reset(bytes, false)
        };

        Self::parse(bytes, parse_state)
    }

    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        let parse_state = |bytes: &[u8], options: &Options| -> (usize, State) {
            match *options.display_type() {
                DisplayType::DelayReset => State::parse_delay_reset(bytes, true),
                DisplayType::Erase => State::parse_base(bytes),
                _ => unreachable!()
            }
        };

        Self::parse(bytes, parse_state)
    }

    pub fn parse_delay(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, |bytes: &[u8], _| State::parse_delay(bytes))
    }

    pub fn parse_range(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, |bytes, _| State::parse_range(bytes))
    }
}