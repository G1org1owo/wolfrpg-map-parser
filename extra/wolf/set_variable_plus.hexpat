#ifndef SET_VARIABLE_PLUS_HEXPAT
#define SET_VARIABLE_PLUS_HEXPAT

namespace SetVariablePlus {
    bitfield Options {
        bind_result: 1;
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

    enum CharacterField: u32 {
        StandardX               = 0x00000000,
        StandardY               = 0x00000001,
        PreciseX                = 0x00000002,
        PreciseY                = 0x00000003,
        HeightOffGround         = 0x00000004,
        Direction               = 0x00000005,
        ScreenX                 = 0x00000006,
        ScreenY                 = 0x00000007,
        ShadowGraphicNumber     = 0x00000008,
        CurrentTileTag          = 0x00000009,
        EventId                 = 0x0000000a,
        OnScreen                = 0x0000000b,
        ActivePage              = 0x0000000c,
        RunCondition            = 0x0000000d,
        RangeExtendX            = 0x0000000e,
        RangeExtendY            = 0x0000000f,
        AnimationPattern        = 0x00000010,
        Moving                  = 0x00000011,
        Unknown
    };

    enum PictureField: u32 {
        PositionX               = 0x00000000,
        PositionY               = 0x00000001,
        Width                   = 0x00000002,
        Height                  = 0x00000003,
        PatternNumber           = 0x00000004,
        Opacity                 = 0x00000005,
        Angle                   = 0x00000006,
        CursorHover             = 0x00000008,
        PictureInUse            = 0x00000009,
        StringDoneDisplaying    = 0x0000000a,
        ZoomWidth               = 0x0000000b,
        ZoomHeight              = 0x0000000c,
        TopLeftX                = 0x0000000d,
        TopLeftY                = 0x0000000e,
        TopRightX               = 0x0000000f,
        TopRightY               = 0x00000010,
        BottomLeftX             = 0x00000011,
        BottomLeftY             = 0x00000012,
        BottomRightX            = 0x00000013,
        BottomRightY            = 0x00000014,
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
                CharacterField field;
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
                PictureField field;
                u16 unknown3;
            }
        }

        u8 end_command;
    };

    enum Target: u32 {
        CurrentMapId            = 0x00000000,
        PlayingBGM              = 0x00000001,
        PlayingBGS              = 0x00000002,
        BGMPosition             = 0x00000003,
        BGSPosition             = 0x00000004,
        CurrentBGMLength        = 0x00000005,
        CurrentBGSLength        = 0x00000006,
        MouseLeftClick          = 0x00000007,
        MouseRightClick         = 0x00000008,
        MouseMiddleClick        = 0x00000009,
        MouseWheelDelta         = 0x0000000a,
        MouseXDelta             = 0x0000000b,
        MouseYDelta             = 0x0000000c,
        EventIDAtMousePosition  = 0x0000000d,
        CallerId                = 0x0000000e,
        MapWidth                = 0x0000000f,
        MapHeight               = 0x00000010,
        ThisCommonId            = 0x00000011,
        ActiveEventId           = 0x00000012,
        ActiveEventLine         = 0x00000013,
        MouseX                  = 0x00000014,
        MouseY                  = 0x00000015,
    };

    struct OtherCommand {
        u8 unknown1;
        u32 variable;
        Options options;
        Assignment assignment;

        u16 unknown2;
        Target target;
        u16 unknown3;

        u8 end_command;
    };
}

#endif