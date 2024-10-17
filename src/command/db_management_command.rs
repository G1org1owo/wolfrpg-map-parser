use serde::Serialize;
use crate::command::db_management_command::base::Base;
use crate::command::db_management_command::csv::CSV;

mod base;
mod options;
mod db_operation_type;
mod db;
mod assignment;
mod assignment_operator;
mod string;
mod csv;

#[derive(Serialize)]
pub enum DBManagementCommand {
    Base(Base),
    String(string::String),
    CSV(CSV)
}

impl DBManagementCommand {
    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Base) = Base::parse(bytes);

        (bytes_read, Self::Base(command))
    }

    pub fn parse_string(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, string::String) = string::String::parse(bytes);

        (bytes_read, Self::String(command))
    }

    pub fn parse_csv(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, CSV) = CSV::parse(bytes);

        (bytes_read, Self::CSV(command))
    }
}