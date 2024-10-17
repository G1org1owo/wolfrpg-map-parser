mod base;
mod options;
mod content_type;
mod variable_type;
mod string_operation;
mod operation;

use serde::Serialize;
use crate::command::set_string_command::base::Base;

#[derive(Serialize)]
pub enum SetStringCommand {
    Base(Base),
    Dynamic
}

impl SetStringCommand {
    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Base) = Base::parse(bytes);

        (bytes_read, Self::Base(command))
    }
}