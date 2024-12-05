mod options;

use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::common::r#move::Move;
use crate::command::event_control_command::move_route::options::Options;

#[derive(Serialize)]
pub struct MoveRoute {
    target: u32,
    unknown1: u32,
    unknown2: u32,
    options: Options,
    move_count: u32,
    moves: Vec<Move>
}

impl MoveRoute {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let target: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let unknown1: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let unknown2: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let options: u8 = bytes[offset];
        let options: Options = Options::new(options);
        offset += 1;

        let move_count: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let (bytes_read, moves): (usize, Vec<Move>)
            = Self::parse_moves(&bytes[offset..], move_count);
        offset += bytes_read;

        (offset, Self {
            target,
            unknown1,
            unknown2,
            options,
            move_count,
            moves
        })
    }

    fn parse_moves(bytes: &[u8], move_count: u32) -> (usize, Vec<Move>) {
        let mut offset: usize = 0;
        let mut moves: Vec<Move> = Vec::with_capacity(move_count as usize);

        for i in 0..move_count {
            let (bytes_read, mov): (usize, Move) = Move::parse(&bytes[offset..]);
            offset += bytes_read;
            moves.push(mov);

        }

        (offset, moves)
    }
}

