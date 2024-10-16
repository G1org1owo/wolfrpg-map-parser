use serde::Serialize;
use crate::command::db_management_command::base::Base;

mod base;
mod options;
mod db_operation_type;
mod db;
mod assignment;
mod assignment_operator;

#[derive(Serialize)]
pub enum DBManagementCommand {
    Base(Base),
    String,
    CSV
}

impl DBManagementCommand {
    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Base) = Base::parse(bytes);

        (bytes_read, Self::Base(command))
    }
}