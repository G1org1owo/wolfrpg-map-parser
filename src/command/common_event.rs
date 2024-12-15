use serde::Serialize;
use crate::command::common_event::event::Event;

mod event;
mod argument_count;
mod options;

#[derive(Serialize)]
pub enum CommonEvent {
    CallEvent(Event),
    ReserveEvent(Event)
}

impl CommonEvent {
    pub fn parse_call_event(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Event) = Event::parse(bytes);

        (bytes_read, Self::CallEvent(command))
    }

    pub fn parse_reserve_event(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Event) = Event::parse(bytes);

        (bytes_read, Self::ReserveEvent(command))
    }
}