use serde::Serialize;
use crate::command::set_string_command::content_type::ContentType;
use crate::command::set_string_command::variable_type::VariableType;

#[derive(Serialize)]
pub struct Options {
    content_type: ContentType,
    variable_type: VariableType,
}

impl Options {
    pub fn new(options: u8) -> Self {
        Self {
            content_type: ContentType::new(options & 0x0f),
            variable_type: VariableType::new(options >> 4 ),
        }
    }
}

