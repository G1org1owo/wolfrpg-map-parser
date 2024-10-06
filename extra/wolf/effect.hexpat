#ifndef EFFECT_HEXPAT
#define EFFECT_HEXPAT

namespace Effect {
    enum EffectTarget: u8 {
        Picture     = 0x00,
        Character   = 0x01,
        Map         = 0x02,
    };

    enum PictureEffectType: u8 {
        Flash                       = 0x00,
        ColorCorrect                = 0x01,
        DrawPositionShift           = 0x02,
        Shake                       = 0x03,
        Zoom                        = 0x04,
        SwitchFlicker               = 0x05,
        SwitchAutoFlash             = 0x06,
        AutoEnlarge                 = 0x07,
        AutoPatternSwitchOne        = 0x08,
        AutoPatternSwitchLoop       = 0x09,
        AutoPatternSwitchRoundTrip  = 0x0A,
    };

    enum CharacterEffectType: u8 {
        Flash           = 0x00,
        Shake           = 0x01,
        SwitchFlicker   = 0x02,
        SwitchAutoFlash = 0x03,
    };

    enum MapEffectType: u8 {
        Zoom    = 0x00,
        Shake   = 0x01,
    };

    bitfield Options {
        EffectTarget target: 4;

        match(target) {
            (EffectTarget::Picture): PictureEffectType type: 4;
            (EffectTarget::Character): CharacterEffectType type: 4;
            (EffectTarget::Map): MapEffectType type: 4;
        }

        padding: 24;
    };

    struct Command {
        u8 unknown1;
        Options options;
        u32 duration;

        u32 target;
        u32 range;

        u32 value1;
        u32 value2;
        u32 value3;

        u24 end_command;
    };
}

namespace EffectMapShake {
    bitfield Options {
        power: 4;
        speed: 4;
    };

    enum Shake: u8 {
        Vertical    = 0x00,
        Horizontal  = 0x01,
        Stop        = 0x02
    };

    struct Command {
        u8 unknown1;
        Options options;
        Shake shake_type;
        u16 unknown2;
        u32 duration;

        u24 end_command;
    };
}

namespace EffectScrollScreen {
    enum ScrollSpeed: u8 {
        OneEight    = 0x00,
        OneFourth   = 0x01,
        OneHalf     = 0x02,
        One         = 0x03,
        Two         = 0x04,
        Four        = 0x05,
        Eight       = 0x06,
        Sixteen     = 0x07,
        Instant     = 0x08,
        ThirtyTwo   = 0x09,
        SixtyFour   = 0x0A,

    };

    enum ScrollOperation: u8 {
        MoveScreen      = 0x00,
        BackToHero      = 0x01,
        LockScroll      = 0x02,
        UnlockScroll    = 0x03,
    };

    bitfield Options {
        ScrollOperation scroll_operation: 4;
        ScrollSpeed scroll_speed: 4;

        bool wait_until_done: 1;
        bool pixel_units: 1;

        padding: 22;
    };

    struct Command {
        u8 unknown1;

        Options options;
        u32 x;
        u32 y;

        u24 end_command;
    };
}

namespace EffectChangeColor {
    struct Command {
        u8 unknown1;

        u8 red;
        u8 green;
        u8 blue;
        bool flash;
        u32 duration;

        u24 end_command;
    };
}

#endif