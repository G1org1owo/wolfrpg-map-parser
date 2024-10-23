use serde::Serialize;
use crate::byte_utils::as_u32_le;
use crate::command::input_key_command::input_toggle::device_inputs::DeviceInputs;

#[derive(Serialize)]
pub struct Device {
    inputs: DeviceInputs,
    enable: bool,
    unknown1: u8,
    key_code: Option<u32>
}

impl Device {
    pub fn parse(bytes: &[u8]) -> (usize, Self) {
        let mut offset: usize = 0;

        let inputs: u8 = bytes[offset];
        let inputs: DeviceInputs = DeviceInputs::new(inputs);
        offset += 1;

        let enable: bool = bytes[offset] == 0;
        offset += 1;

        let unknown1: u8 = bytes[offset];
        offset += 1;

        offset += 1; // input_type

        let key_code: Option<u32> = match inputs {
            DeviceInputs::KeyboardKey => {
                let key_code = as_u32_le(&bytes[offset..offset + 4]);
                offset += 4;

                Some(key_code)
            }

            _ => None,
        };

        (offset, Self {
            inputs,
            enable,
            unknown1,
            key_code
        })
    }
}