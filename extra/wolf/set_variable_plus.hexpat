#ifndef SET_VARIABLE_PLUS_HEXPAT
#define SET_VARIABLE_PLUS_HEXPAT

namespace SetVariablePlus {
    bitfield Options {
        bound_result: 1;
        padding: 3;
        use_variable_as_reference: 1;
        precise_position: 1;
        padding: 2;
    };

    enum Type: u8 {
        Character       = 0b00000001,
        Position        = 0b00000010,
        Other           = 0b00000011,
        PictureNumber   = 0b00000100,
    };

    enum AssignmentOperator: u8 {
        Equals          = 0x00,
        PlusEquals      = 0x01,
        MinusEquals     = 0x02,
        TimesEquals     = 0x03,
        DivideEquals    = 0x04,
        RemainderEquals = 0x05,
        LowerBound      = 0x06,
        UpperBound      = 0x07,
        Absolute        = 0x08,
    };

    bitfield Assignment {
        AssignmentOperator operator: 4;
        Type variable_type: 4;
    };

    struct Command {
        u8 unknown1;
        u32 variable;
        Options options;
        Assignment assignment;

        match(assignment.variable_type) {
            (Type::Character): {
                u16 unknown2;
                u32 character;
                u32 field;
                u16 unknown3;
            }
            (Type::Position): {
                u8 target;
                u8 unknown2;
                u32 position_x;
                u32 position_y;
                u16 unknown3;
            }
            (Type::PictureNumber): {
                u16 unknown2;
                u32 picture_number;
                u32 field;
                u16 unknown3;
            }
        }

        u8 end_command;
    };

    struct OtherCommand {
        u8 unknown1;
        u32 variable;
        Options options;
        Assignment assignment;

        u16 unknown2;
        u32 target;
        u16 unknown3;

        u8 end_command;
    };
}

#endif