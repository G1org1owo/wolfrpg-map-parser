#ifndef INPUT_KEY_HEXPAT
#define INPUT_KEY_HEXPAT

namespace InputKey {
    enum DirectionKeys: u8 {
        No          = 0x00,
        Dir4        = 0x01,
        Dir8        = 0x02,
        Up          = 0x03,
        Down        = 0x04,
        Left        = 0x05,
        Right       = 0x06,
        UpDown      = 0x07,
        LeftRight   = 0x08,
    };

    bitfield BasicOptions {
        DirectionKeys direction_keys: 4;
        input_ok: 1;
        input_cancel: 1;
        input_subkey: 1;
        wait_for_input: 1;
    };

    enum MouseTarget: u8 {
        Click       = 0x00,
        XPos        = 0x01,
        YPos        = 0x02,
        WheelDelta  = 0x03,
    };

    bitfield MouseOptions {
        MouseTarget target: 4;
        left_click: 1;
        right_click: 1;
        middle_click: 1;
        wait_for_input: 1;
    };

    bitfield KeyOptions {
        padding: 7;
        wait_for_input: 1;
    };

    enum InputType: u8 {
        Basic       = 0x00,
        KeyBoard    = 0x01,
        Pad         = 0x02,
        Mouse       = 0x03,
    };

    bitfield Input {
        InputType input_type: 4;
        specific_key: 1;
        padding: 3;
    };

    fn read_input_type(u32 address) {
        Input input_type @ address;
        return input_type.input_type;
    };

    struct Command {
        u8 unknown1;
        u32 variable;

        match(InputKey::read_input_type($ + 1)) {
            (InputType::Basic): {
                BasicOptions options;
                InputType input_type;
                u32 unknown2;
            }

            (InputType::KeyBoard | InputType::Pad): {
                KeyOptions options;
                Input input_type;
                u16 unknown2;
                u32 key_code;
                u16 unknown3;
            }

            (InputType::Mouse): {
                MouseOptions options;
                InputType input_type;
                u32 unknown2;
            }
        }

        u8 end_command;
    };
}

namespace AutomaticInput {
    bitfield BasicOptions {
        input_ok: 1;
        input_cancel: 1;
        input_subkey: 1;
        padding: 1;
        down: 1;
        left: 1;
        right: 1;
        up: 1;
    };

    enum InputType: u8 {
        Basic       = 0x00,
        KeyBoard    = 0x10,
        Mouse       = 0x20,
    };

    enum MouseType: u8 {
        Position    = 0x01,
        Wheel       = 0x02,
    };

    bitfield MouseOptions {
        left_click: 1;
        right_click: 1;
        middle_click: 1;
        MouseType mouse_type: 5;
    };

    fn read_input_type(u32 address) {
        InputType input_type @ address;
        return input_type;
    };

    struct Command {
        u8 unknown1;

        match(AutomaticInput::read_input_type($+3)) {
            (InputType::Basic): {
                BasicOptions options;
                u16 unknown2;
                InputType type;
            }

            (InputType::KeyBoard): {
                u24 unknown2;
                InputType type;
                u32 key_code;
            }

            (InputType::Mouse): {
                MouseOptions options;
                u16 unknown2;
                InputType type;

                if(options.mouse_type == MouseType::Position) {
                    u32 position_x;
                    u32 position_y;
                }

                if(options.mouse_type == MouseType::Wheel) {
                    u32 wheel_delta;
                    u32 unknown3;
                }
            }
        }

        u24 command_end;
    };
}

namespace InputToggle {
    enum InputType: u8 {
        Basic   = 0x00,
        Device  = 0x10,
    };

    bitfield BasicInputs {
        Ok: 1;
        Cancel: 1;
        Sub: 1;
        padding: 1;
        Down: 1;
        Left: 1;
        Right: 1;
        Up: 1;
    };

    enum EnabledInputs: u8 {
        AllEnabled  = 0x00,
        NoMovement  = 0x01,
        AllDisabled = 0x02,
    };

    enum DeviceInputs: u8 {
        KeyboardKey     = 0x00,
        AllMouseInput   = 0x01,
        AllPadInput     = 0x02,
        AllDevices      = 0x03,
    };

    fn read_input_type(u32 address) {
        InputType input_type @ address;
        return input_type;
    };

    struct Command {
        u8 unknown1;

        match(InputToggle::read_input_type($+3)) {
            (InputType::Basic): {
                BasicInputs inputs;
                EnabledInputs enabled_inputs;
                u8 unknown2;
                InputType input_type;
            }

            (InputType::Device): {
                DeviceInputs inputs;
                bool disable;
                u8 unknown2;
                InputType input_type;

                if(inputs == DeviceInputs::KeyboardKey) {
                    u32 key_code;
                }
            }
        }

        u24 command_end;
    };
}

#endif