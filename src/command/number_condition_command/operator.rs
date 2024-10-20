use serde::Serialize;
use crate::command::number_condition_command::compare_operator::CompareOperator;

#[derive(Serialize)]
pub struct Operator {
    operator: CompareOperator,
    not_variable: bool,
}

impl Operator {
    pub fn new(operator: u8) -> Self {
        Self {
            operator: CompareOperator::new(operator & 0x0f),
            not_variable: operator & 0b00010000 != 0,
        }
    }
}
