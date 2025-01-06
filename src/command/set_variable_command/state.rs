use crate::command::set_variable_command::base::Base;
use crate::command::set_variable_command::range::Range;
#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum State {
    Base(Base),
    Range(Range)
}

impl State {
    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Base)
            = Base::parse(bytes);

        (bytes_read, Self::Base(command))
    }

    pub fn parse_range(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Range)
            = Range::parse(bytes);

        (bytes_read, Self::Range(command))
    }
}