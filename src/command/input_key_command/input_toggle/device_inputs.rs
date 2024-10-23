use serde::Serialize;

#[derive(Serialize)]
pub enum DeviceInputs {
    KeyboardKey     = 0x00,
    AllMouseInput   = 0x01,
    AllPadInput     = 0x02,
    AllDevices      = 0x03,
    Unknown
}

impl DeviceInputs {
    pub const fn new(inputs: u8) -> Self {
        match inputs {
            0x00 => DeviceInputs::KeyboardKey,
            0x01 => DeviceInputs::AllMouseInput,
            0x02 => DeviceInputs::AllPadInput,
            0x03 => DeviceInputs::AllDevices,
            _ => DeviceInputs::Unknown
        }
    }
}
