#ifndef CHIP_MANAGEMENT_HEXPAT
#define CHIP_MANAGEMENT_HEXPAT

namespace MapChip {
    bitfield Options {
        bool no_down: 1;
        bool no_left: 1;
        bool no_right: 1;
        bool no_up: 1;
        bool above_hero: 1;
        padding: 1;
        bool half_transparent: 1;
        bool counter: 1;

        padding: 1;
        bool match_lower_layer: 1;
        padding: 6;

        padding: 16;
    };

    struct SettingsCommand {
        u8 unknown1;
        u32 chip;
        Options options;

        u24 end_command;
    };

    struct SwitchChipsetCommand {
        u8 unknown1;
        u32 chipset;

        u24 end_command;
    };

    struct OverwriteChipsCommand {
        u8 unknown1;
        u32 layer;
        u32 position_x;
        u32 position_y;
        u32 width;
        u32 height;
        u32 chip;

        u24 end_command;
    };
}

#endif