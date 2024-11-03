use serde::Serialize;
use crate::command::party_graphics_command::operation::Operation;
use crate::command::party_graphics_command::special_operation::SpecialOperation;

#[derive(Serialize)]
pub struct Options {
    operation: Operation,
    special_operation: SpecialOperation,
    is_graphics_variable: bool
}

impl Options {
    pub fn new(options: u32) -> Self {
        Self {
            operation: Operation::new((options & 0x0f) as u8),
            special_operation: SpecialOperation::new(((options >> 4) & 0x0f) as u8),
            is_graphics_variable: (options >> 8) & 0b00000001 != 0
        }
    }

    pub fn operation(&self) -> &Operation {
        &self.operation
    }

    pub fn special_operation(&self) -> &SpecialOperation {
        &self.special_operation
    }

    pub fn is_graphics_variable(&self) -> bool {
        self.is_graphics_variable
    }
}