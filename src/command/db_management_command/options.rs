use serde::Serialize;
use crate::command::db_management_command::db::DB;
use crate::command::db_management_command::db_operation_type::DBOperationType;

#[derive(Serialize)]
pub struct Options {
    db: DB,
    db_operation_type: DBOperationType
}

impl Options {
    pub fn new(options: u8) -> Self {
        Self {
            db: DB::new(options & 0x0f),
            db_operation_type: DBOperationType::new(options >> 4),
        }
    }
}