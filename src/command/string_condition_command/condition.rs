use serde::Serialize;
use crate::command::common::u32_or_string::U32OrString;
use crate::command::string_condition_command::operator::Operator;

#[derive(Serialize)]
pub struct Condition {
    variable: u32,
    operator: Operator,
    value: U32OrString
}

impl Condition {
    pub fn new(variable: u32, operator: Operator, value: U32OrString) -> Condition {
        Condition {
            variable,
            operator,
            value
        }
    }
}