use serde::Serialize;
use crate::command::db_management_command::assignment_operator::AssignmentOperator;

#[derive(Serialize)]
pub struct Assignment {
    use_variable_as_reference: bool,
    assignment_operator: AssignmentOperator
}

impl Assignment {
    pub fn new(assignment: u8) -> Self {
        Self {
            use_variable_as_reference: assignment & 0b00000001 != 0,
            assignment_operator: AssignmentOperator::new(assignment >> 4)
        }
    }
}