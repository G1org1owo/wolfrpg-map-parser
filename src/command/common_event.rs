use serde::Serialize;
use crate::command::common_event::call_event::CallEvent;

mod call_event;
mod argument_count;
mod options;

#[derive(Serialize)]
pub enum CommonEvent {
    CallEvent(CallEvent),
    CallEventByVariable,
    ReserveEvent
}

impl CommonEvent {
    pub fn parse_call_event(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, CallEvent) = CallEvent::parse(bytes);

        (bytes_read, Self::CallEvent(command))
    }
}