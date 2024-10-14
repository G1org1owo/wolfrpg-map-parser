use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::Command;

#[derive(Serialize)]
pub struct Condition {
    operator: u8, // bitmask
    variable: u32,
    value: u32,
}

#[derive(Serialize)]
pub struct Page {
    icon: String,
    icon_row: u8,
    icon_column: u8,
    icon_opacity: u8,
    icon_blend: u8,
    event_trigger: u8,
    conditions: [Condition; 4],
    animation_speed: u8,
    move_speed: u8,
    move_frequency: u8,
    move_route: u8,
    options: u8, // bitmask
    unknown2: u8,
    move_count: u32,
    command_count: u32,
    commands: Vec<Command>,
    unknown4: u32,
    shadow_graphic: u8,
    range_extension_x: u8,
    range_extension_y: u8,
}

impl Page {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let signature = &bytes[offset..offset+5];
        offset+=5;
        //TODO: Throw error if signature is not 0x79ffffffff

        let icon_length: usize = as_u32_le(&bytes[offset..offset+4]).try_into().unwrap();
        offset+=4;

        let icon: String = String::from_utf8(bytes[offset..offset+icon_length-1].to_vec())
            .unwrap();
        offset+=icon_length;

        let icon_row: u8 = bytes[offset];
        let icon_column: u8 = bytes[offset+1];
        let icon_opacity: u8 = bytes[offset+2];
        let icon_blend: u8 = bytes[offset+3];
        let event_trigger: u8 = bytes[offset+4];
        offset+=5;

        let condition1: Condition = Self::parse_condition(&bytes[offset..], 0);
        let condition2: Condition = Self::parse_condition(&bytes[offset..], 1);
        let condition3: Condition = Self::parse_condition(&bytes[offset..], 2);
        let condition4: Condition = Self::parse_condition(&bytes[offset..], 3);
        offset+=36;

        let animation_speed: u8 = bytes[offset];
        let move_speed: u8 = bytes[offset+1];
        let move_frequency: u8 = bytes[offset+2];
        let move_route: u8 = bytes[offset+3];
        let options: u8 = bytes[offset+4];
        let unknown2: u8 = bytes[offset+5];
        let move_count: u32 = as_u32_le(&bytes[offset+6..offset+10]);
        offset+=10;

        // TODO: Parse moves

        let command_count = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let (bytes_read, commands): (usize, Vec<Command>)
            = Self::parse_commands(&bytes[offset..], command_count);
        offset += bytes_read;

        let unknown4: u32 = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let shadow_graphic: u8 = bytes[offset];
        let range_extension_x: u8 = bytes[offset+1];
        let range_extension_y: u8 = bytes[offset+2];
        offset += 3;

        let page_end: u8 = bytes[offset]; // TODO: Throw error if not 0x7a
        offset += 1;

        (offset, Self {
            icon,
            icon_row,
            icon_column,
            icon_opacity,
            icon_blend,
            event_trigger,
            conditions: [condition1, condition2, condition3, condition4],
            animation_speed,
            move_speed,
            move_frequency,
            move_route,
            options,
            unknown2,
            move_count,
            command_count,
            commands,
            unknown4,
            shadow_graphic,
            range_extension_x,
            range_extension_y,
        })
    }

    fn parse_condition(bytes: &[u8], count: usize) -> Condition {
        Condition {
            operator: bytes[count],
            variable: as_u32_le(&bytes[4*(1+count)..4*(2+count)]),
            value: as_u32_le(&bytes[4*(5+count)..4*(6+count)]),
        }
    }

    fn parse_commands(bytes: &[u8], command_count: u32) -> (usize, Vec<Command>){
        let mut offset: usize = 0;
        let mut commands: Vec<Command> = Vec::new();

        let mut i: u32 = 0;
        while i<command_count {
            let (bytes_read, commands_read, command): (usize, u32, Command) = Command::parse(&bytes[offset..]);
            offset += bytes_read;
            commands.push(command);

            i += commands_read;
        }

        (offset, commands)
    }
}