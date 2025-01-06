#[cfg(feature = "serde")]
use serde::Serialize;
use crate::command::save_load_command::base::Base;
use crate::command::save_load_command::load_variable::LoadVariable;
use crate::command::save_load_command::save_variable::SaveVariable;

pub mod base;
pub mod operation;
pub mod save_variable;
mod parser;
pub mod load_variable;

#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SaveLoadCommand {
    Base(Base),
    LoadVariable(LoadVariable),
    SaveVariable(SaveVariable),
}

impl SaveLoadCommand {
    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, state): (usize, Base) = Base::parse(bytes);

        (bytes_read, Self::Base(state))
    }

    pub fn parse_load_variable(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, state): (usize, LoadVariable) = LoadVariable::parse(bytes);

        (bytes_read, Self::LoadVariable(state))
    }

    pub fn parse_save_variable(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, state): (usize, SaveVariable) = SaveVariable::parse(bytes);

        (bytes_read, Self::SaveVariable(state))
    }
}