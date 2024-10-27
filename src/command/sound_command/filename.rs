use serde::Serialize;
use crate::byte_utils::{as_string, as_u32_le};
use crate::command::sound_command::operation::Operation;
use crate::command::sound_command::options::Options;
use crate::command::sound_command::variable::Variable;

#[derive(Serialize)]
pub struct Filename {
    variable_state: Variable,
    volume: u32,
    tempo: u32,
    loop_point: Option<u32>,
    sound_filename: String
}

impl Filename {
    pub fn parse(bytes: &[u8], options: &Options) -> (usize, Self) {
        let mut offset: usize = 0;

        let (bytes_read, variable_state): (usize, Variable) = Variable::parse(bytes, options);
        offset += bytes_read;

        let volume: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let tempo: u32 = as_u32_le(&bytes[offset..offset + 4]);
        offset += 4;

        let loop_point: Option<u32> = match *options.operation() {
            Operation::SetSE => None,
            _ => {
                let loop_point = as_u32_le(&bytes[offset..offset + 4]);
                offset += 4;

                Some(loop_point)
            }
        };

        let sound_length: usize = as_u32_le(&bytes[offset..offset + 4]) as usize;
        offset += 4;
        let sound_filename: String = as_string(bytes, offset, sound_length);
        offset += sound_length;

        (offset, Self {
            variable_state,
            volume,
            tempo,
            loop_point,
            sound_filename
        })
    }
}