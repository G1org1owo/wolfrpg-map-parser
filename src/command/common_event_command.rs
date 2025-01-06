#[cfg(feature = "serde")]
use serde::Serialize;
use crate::command::common_event_command::event::Event;

pub mod event;
pub mod argument_count;
pub mod options;

#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum CommonEventCommand {
    CallEvent(Event),
    ReserveEvent(Event)
}

impl CommonEventCommand {
    pub fn parse_call_event(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Event) = Event::parse(bytes);

        (bytes_read, Self::CallEvent(command))
    }

    pub fn parse_reserve_event(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Event) = Event::parse(bytes);

        (bytes_read, Self::ReserveEvent(command))
    }
}