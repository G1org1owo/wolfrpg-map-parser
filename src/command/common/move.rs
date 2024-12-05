use serde::Serialize;
use move_type::MoveType;
use crate::byte_utils::as_u16_le;
use crate::command::common::r#move::state::State;

mod move_type;
mod state;

#[derive(Serialize)]
pub struct Move {
    move_type: MoveType,
    state: State
}

impl Move {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let move_type: u16 = as_u16_le(&bytes[offset..offset + 2]);
        let move_type: MoveType = MoveType::new(move_type);
        offset += 2;

        let (bytes_read, state): (usize, State) = State::parse(&bytes[offset..], &move_type);
        offset += bytes_read;

        offset += 2; // Move end signature

        (offset, Move {
            move_type,
            state
        })
    }
}