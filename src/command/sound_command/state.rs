use serde::Serialize;
use crate::command::sound_command::filename::Filename;
use crate::command::sound_command::options::Options;

#[derive(Serialize)]
pub enum State {
    Filename(Filename),
    Variable,
    FreeAll
}

impl State {
    pub fn parse_filename(bytes: &[u8], options: &Options) -> (usize, Self) {
        let (bytes_read, state): (usize, Filename) = Filename::parse(bytes, options);

        (bytes_read, Self::Filename(state))
    }
}