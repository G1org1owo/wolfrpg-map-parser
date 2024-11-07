use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::transfer_command::options::Options;
use crate::command::transfer_command::target::Target;

mod target;
mod options;
mod transition;

#[derive(Serialize)]
pub struct TransferCommand {
    target: Target,
    db_variable: Option<u32>,
    destination_x: u32,
    destination_y: u32,
    destination_map: Option<u32>,
    options: Options
}

impl TransferCommand {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let target: u32 = as_u32_le(&bytes[offset..offset+4]);
        let target: Target = Target::new(target);
        offset += 4;

        let db_variable: Option<u32> = match target {
            Target::SavedPosition => {
                let db_variable: u32 = as_u32_le(&bytes[offset..offset+4]);
                offset += 4;

                Some(db_variable)
            },
            _ => None
        };

        let destination_x: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let destination_y: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let destination_map: Option<u32> = match target {
            Target::SavedPosition => None,
            _ => {
                let destination_map: u32 = as_u32_le(&bytes[offset..offset+4]);
                offset += 4;

                Some(destination_map)
            }
        };

        let options: u32 = as_u32_le(&bytes[offset..offset+4]);
        let options: Options = Options::new(options);
        offset += 4;

        offset += 3; // Command end signature

        (offset, Self {
            target,
            db_variable,
            destination_x,
            destination_y,
            destination_map,
            options
        })
    }
}