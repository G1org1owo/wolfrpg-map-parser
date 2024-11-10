use serde::Serialize;
use crate::command::event_control_command::loop_command::Loop;

mod loop_command;

#[derive(Serialize)]
pub enum EventControlCommand {
    Loop(Loop),
    BreakLoop,
    GotoLoopStart,
    PrepareTransition,
    ExecuteTransition,
    SetTransition,
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
    pub fn parse_loop(bytes: &[u8]) -> (usize, u32, Self) {
        let (bytes_read, commands_read, command): (usize, u32, Loop) = Loop::parse(bytes);

        (bytes_read, commands_read, Self::Loop(command))
    }
}