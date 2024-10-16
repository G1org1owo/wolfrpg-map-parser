use serde::Serialize;
use crate::command::set_variable_command::assignment::Assignment;
use crate::command::set_variable_command::calculation::Calculation;

#[derive(Serialize)]
pub struct Operators {
    assignment: Assignment,
    calculation: Calculation,
}

impl Operators {
    pub fn new(operators: u8) -> Self {
        Self {
            assignment: Assignment::from_u8(operators & 0x0f),
            calculation: Calculation::from_u8(operators >> 4),
        }
    }
}