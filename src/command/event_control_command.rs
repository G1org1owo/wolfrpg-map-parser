use serde::Serialize;
use crate::command::event_control_command::loop_command::Loop;
use crate::command::event_control_command::set_transition::SetTransition;

mod loop_command;
mod set_transition;

const COMMAND_END_SIGNATURE_LENGTH: usize = 3;

#[derive(Serialize)]
pub enum EventControlCommand {
    Loop(Loop),
    BreakLoop,
    GotoLoopStart,
    PrepareTransition,
    ExecuteTransition,
    SetTransition(SetTransition),
    MoveRoute,
    WaitForMoveRoute,
    MoveDuringEventsOn,
    MoveDuringEventsOff,
    GotoTitle,
    GameEnd,
    StopNonPictureGraphicUpdates,
    ResumeNonPictureGraphicUpdates,
    ForceExitEvent,
    EraseEvent,
    WaitEvent,
    LoopForCount,
    LabelPoint,
    LabelJump
}

impl EventControlCommand {
    fn parse_empty_command(command: EventControlCommand) -> (usize, Self) {
        (COMMAND_END_SIGNATURE_LENGTH, command)
    }
    pub fn parse_loop(bytes: &[u8]) -> (usize, u32, Self) {
        let (bytes_read, commands_read, command): (usize, u32, Loop) = Loop::parse(bytes);

        (bytes_read, commands_read, Self::Loop(command))
    }

    pub fn parse_break_loop(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::BreakLoop)
    }

    pub fn parse_goto_loop_start(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::GotoLoopStart)
    }

    pub fn parse_prepare_transition(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::PrepareTransition)
    }

    pub fn parse_execute_transition(bytes: &[u8]) -> (usize, Self) {
        Self::parse_empty_command(Self::ExecuteTransition)
    }

    pub fn parse_set_transition(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, SetTransition) = SetTransition::parse(bytes);

        (bytes_read, Self::SetTransition(command))
    }
}