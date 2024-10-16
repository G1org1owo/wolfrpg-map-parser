use serde::Serialize;
use crate::command::set_variable_command::base::Base;
use crate::command::set_variable_command::range::Range;

mod base;
mod assignment;
mod calculation;
mod options;
mod operators;
mod range;

#[derive(Serialize)]
pub enum SetVariableCommand {
    Base(Base),
    Range(Range)
}

impl SetVariableCommand {
    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Base)
            = Base::parse(bytes);

        (bytes_read, SetVariableCommand::Base(command))
    }

    pub fn parse_range(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Range)
            = Range::parse(bytes);

        (bytes_read, SetVariableCommand::Range(command))
    }
}
