use serde::Serialize;
use crate::byte_utils::as_u16_le;
use crate::command::sound_command::options::Options;
use crate::command::sound_command::sound_type::SoundType;
use crate::command::sound_command::state::State;

mod options;
mod process_type;
mod operation;
mod sound_type;
mod state;
mod filename;

#[derive(Serialize)]
pub struct SoundCommand {
    options: Options,
    systemdb_entry: u16,
    sound_type: SoundType,
    state: State
}

impl SoundCommand {
    fn parse(bytes: &[u8], parse_state: fn(&[u8], &Options) -> (usize, State)) -> (usize, Self) {
        let mut offset: usize = 0;

        let options: u8 = bytes[offset];
        let options: Options = Options::new(options);
        offset += 1;

        let systemdb_entry: u16 = as_u16_le(&bytes[offset..offset + 2]);
        offset += 2;

        let sound_type: SoundType = SoundType::new(bytes[offset]);
        offset += 1;

        let (bytes_read, state): (usize, State) = parse_state(&bytes[offset..], &options);
        offset += bytes_read;

        offset += 1; // Command end signature

        (offset, Self {
            options,
            systemdb_entry,
            sound_type,
            state
        })
    }

    pub fn parse_filename(bytes: &[u8]) -> (usize, Self) {
        Self::parse(bytes, State::parse_filename)
    }
}