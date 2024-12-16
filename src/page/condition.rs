use serde::Serialize;
use crate::common::compare_operator::CompareOperator;

#[derive(Serialize)]
pub struct Condition {
    operator: CompareOperator,
    variable: u32,
    value: u32,
}

impl Condition {
    pub fn new(operator: u8, variable: u32, value: u32) -> Self {
        Self {
            operator: CompareOperator::new(operator >> 4),
            variable,
            value
        }
    }

    pub fn operator(&self) -> &CompareOperator {
        &self.operator
    }

    pub fn operator_mut(&mut self) -> &mut CompareOperator {
        &mut self.operator
    }

    pub fn variable(&self) -> u32 {
        self.variable
    }

    pub fn variable_mut(&mut self) -> &mut u32 {
        &mut self.variable
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn value_mut(&mut self) -> &mut u32 {
        &mut self.value
    }
}