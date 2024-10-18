use serde::Serialize;
use crate::command::set_string_command::base::Base;

#[derive(Serialize)]
pub enum SetStringCommandState {
    Base(Base),
    Dynamic
}

impl SetStringCommandState {
    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Base) = Base::parse(bytes);

        (bytes_read, Self::Base(command))
    }
}