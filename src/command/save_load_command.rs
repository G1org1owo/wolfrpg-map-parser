use serde::Serialize;
use crate::command::save_load_command::base::Base;

mod base;
mod operation;

#[derive(Serialize)]
pub enum SaveLoadCommand {
    Base(Base),
    SaveVariable,
    LoadVariable
}

impl SaveLoadCommand {
    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, state): (usize, Base) = Base::parse(bytes);

        (bytes_read, Self::Base(state))
    }
}