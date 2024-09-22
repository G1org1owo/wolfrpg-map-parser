use serde::Serialize;
use crate::byte_utils::{as_u32_le};
use crate::command::Command;

#[derive(Serialize)]
pub struct Event {
    name: String,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    icon: String,
    unknown5: Vec<u8>,
    command_count: u32,
    commands: Vec<Command>
}

impl Event {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset = 9;
        let name_length: usize = as_u32_le(&bytes[offset..offset+4]) as usize;
        offset+=4;

        println!("{}/{}", offset, name_length);
        for i in bytes[0..9].iter() {
            print!("{:02x}", i);
        }
        println!();

        let name = String::from_utf8(bytes[offset..offset+name_length-1].to_vec())
            .unwrap();
        offset+=name_length;

        let unknown1 = as_u32_le(&bytes[offset..offset+4]);
        let unknown2 = as_u32_le(&bytes[offset+4..offset+8]);
        let unknown3 = as_u32_le(&bytes[offset+8..offset+12]);
        let unknown4 = as_u32_le(&bytes[offset+12..offset+16]);
        offset += 16;

        offset+=5;
        let icon_length: usize = as_u32_le(&bytes[offset..offset+4]).try_into().unwrap();
        offset+=4;

        let icon = String::from_utf8(bytes[offset..offset+icon_length-1].to_vec())
            .unwrap();
        offset+=icon_length;

        let unknown5: Vec<u8> = bytes[offset..offset+51].to_vec();
        offset+=51;

        let command_count = as_u32_le(&bytes[offset..offset+4]);
        offset += 4;

        let (bytes_read, commands): (usize, Vec<Command>)
            = Event::parse_commands(&bytes[offset..], command_count);
        offset += bytes_read;

        offset+=4; // TODO: throw error if not event end signature

        (offset, Self {
            name,
            unknown1,
            unknown2,
            unknown3,
            unknown4,
            icon,
            unknown5,
            command_count,
            commands
        })
    }

    fn parse_commands(bytes: &[u8], command_count: u32) -> (usize, Vec<Command>){
        let mut offset: usize = 0;
        let mut commands: Vec<Command> = Vec::new();

        for _i in 0..command_count {
            let (bytes_read, command): (usize, Command) = Command::parse(&bytes[offset..]);
            offset += bytes_read;
            commands.push(command);
        }

        (offset, commands)
    }
}