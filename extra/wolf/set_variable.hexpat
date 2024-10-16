#ifndef SET_VARIABLE_HEXPAT
#define SET_VARIABLE_HEXPAT

namespace SetVariable {
    bitfield Options {
        bind_result: 1;
        real_number_calculation: 1;
        left_not_variable: 1;
        right_not_variable: 1;
        use_variable_as_reference: 1;
        use_left_as_reference: 1;
        use_right_as_reference: 1;
        padding: 1;
    };

    enum Assignment: u8 {
        Equals          = 0x00,
        PlusEquals      = 0x01,
        MinusEquals     = 0x02,
        TimesEquals     = 0x03,
        DivideEquals    = 0x04,
        RemainderEquals = 0x05,
        LowerBound      = 0x06,
        UpperBound      = 0x07,
        Absolute        = 0x08,
        Angle           = 0x09,
        Sin             = 0x0a,
        Cos             = 0x0b,
    };

    enum Calculation: u8 {
        Plus        = 0x00,
        Minus       = 0x01,
        Times       = 0x02,
        Divides     = 0x03,
        Remainder   = 0x04,
        BitwiseAnd  = 0x05,
        BitwiseNot  = 0x06,
    };

    bitfield Operators {
        Assignment assignment: 4;
        Calculation calculation: 4;
    };

    struct Command {
        u8 unknown1;
        u32 variable;
        u32 left_side;
        u32 right_side;
        Options options;
        Operators operators;
        u32 unknown2;
        u8 end_command;
    };

    struct RangeCommand {
        u8 unknown1;
        u32 variable;
        u32 left_side;
        u32 right_side;
        Options options;
        Operators operators;
        u16 range_start;
        u16 range_end;
        u32 unknown2;
        u8 end_command;
    };
}

#endif