use serde::Serialize;
use crate::command::string_condition_command::compare_operator::CompareOperator;

#[derive(Serialize)]
pub struct Operator {
    is_value_variable: bool,
    operator: CompareOperator
}

impl Operator {
    pub fn new(operator: u8) -> Self {
        Self {
            is_value_variable: operator & 0b00000001 != 0,
            operator: CompareOperator::new(operator >> 4)
        }
    }

    pub fn is_value_variable(&self) -> &bool {
        &self.is_value_variable
    }

    pub fn operator(&self) -> &CompareOperator {
        &self.operator
    }

    pub fn operator_mut(&mut self) -> &mut CompareOperator {
        &mut self.operator
    }
}

