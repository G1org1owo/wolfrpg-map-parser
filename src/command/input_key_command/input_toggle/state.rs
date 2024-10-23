use serde::Serialize;
use crate::command::input_key_command::input_toggle::basic::Basic;
use crate::command::input_key_command::input_toggle::device::Device;
use crate::command::input_key_command::input_toggle::input_type::InputType;

#[derive(Serialize)]
pub enum State {
    Basic(Basic),
    Device(Device),
}

impl State {
    pub fn parse(bytes: &[u8], input_type: &InputType) -> (usize, Self) {
        match *input_type {
            InputType::Basic => {
                let (bytes_read, state): (usize, Basic) = Basic::parse(bytes);

                (bytes_read, Self::Basic(state))
            }

            InputType::Device => {
                let (bytes_read, device): (usize, Device) = Device::parse(bytes);

                (bytes_read, Self::Device(device))
            }

            _ => unreachable!(),
        }
    }
}