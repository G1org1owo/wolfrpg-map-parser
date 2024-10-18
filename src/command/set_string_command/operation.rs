use serde::Serialize;
use crate::command::set_string_command::string_operation::StringOperation;

#[derive(Serialize)]
pub struct Operation {
    operation: StringOperation,
    input_cancel: bool,
    input_replace: bool,
}

impl Operation {
    pub fn new(operation: u8) -> Self {
        Self {
            operation: StringOperation::new(operation & 0x0f),
            input_replace: operation & 0b00010000 != 0,
            input_cancel: operation  & 0b00100000 != 0,
        }
    }
}