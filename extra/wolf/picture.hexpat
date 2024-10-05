#ifndef PICTURE_HEXPAT
#define PICTURE_HEXPAT

namespace Picture {
    enum DisplayType: u8 {
        LoadFile            = 0x00,
        Move                = 0x01,
        Erase               = 0x02,
        DelayReset          = 0x03,
        LoadFileByStringVar = 0x10,
        ShowStringAsPicture = 0x20,
        Window              = 0x30,
        WindowByStringVar   = 0x40,
    };

    enum Anchor: u8 {
        TopLeft     = 0x00,
        Center      = 0x01,
        BottomLeft  = 0x02,
        TopRight    = 0x03,
        BottomRight = 0x04,
    };

    enum BlendingMethod: u8 {
        Normal      = 0x00,
        Add         = 0x01,
        Subtract    = 0x02,
        Multiply    = 0x03,
        Same        = 0x0F,
    };

    enum Zoom: u8 {
        Normal      = 0x00,
        Different   = 0x03,
        Same        = 0x04,
    };

    bitfield Options {
        DisplayType display_type: 8;

        BlendingMethod blending_method: 4;
        Anchor anchor: 4;

        bool position_relative: 1;
        padding: 3;
        Zoom zoom: 4;

        bool range: 1;
        bool link_to_scroll: 1;
        bool free_transform: 1;
        padding: 5;
    };

    struct ShowCommand {
        u8 unknown1;
        Options options;
        u32 picture;
        u32 process_time;
        u32 division_width;
        u32 division_height;
        u32 pattern;
        u32 opacity;
        u32 position_x;
        u32 position_y;

        u32 zoom;
        u32 angle;

        u8 unknown2;
        bool is_filename_string;

        if(is_filename_string) {
            u32 filename_length;
            char filename[filename_length];
        }

        u8 end_command;
    };

    enum Colors: u8 {
        Same        = 0x01,
        Different   = 0x02,
    };

    struct ShowCompleteCommand {
        u8 unknown1;
        Options options;
        u32 picture;
        u32 process_time;
        u32 division_width;
        u32 division_height;
        u32 pattern;
        u32 opacity;
        if(options.free_transform) {
            u32 top_left_x;
            u32 top_left_y;
        } else {
            u32 position_x;
            u32 position_y;
        }

        u32 zoom;
        u32 angle;

        u32 filename_variable;

        u24 unknown2;
        Colors colors;

        if(colors == Colors::Different || options.free_transform || options.zoom == Zoom::Different) {
            u32 delay;
            u32 range_count;
            u32 color1;
            u32 color2;
            u32 color3;
        }

        if(options.zoom == Zoom::Different || options.free_transform) {
            u32 zoom_height;
        }

        if(options.free_transform) {
            u32 top_right_x;
            u32 top_right_y;

            u32 bottom_left_x;
            u32 bottom_left_y;

            u32 bottom_right_x;
            u32 bottom_right_y;
        }

        u8 unknown4;
        bool is_filename_string;

        if(is_filename_string) {
            u32 filename_length;
            char filename[filename_length];
        }

        u8 end_command;
    };

    struct EraseCommand {
        u8 unknown1;
        Options options;
        u32 picture;

        match(options.display_type) {
            (DisplayType::Erase): u32 process_time;
            (DisplayType::DelayReset): {
                if(options.range) {
                    u32 range_count;
                }
            }
        }

        u24 end_command;
    };

    struct EraseCompleteCommand {
        u8 unknown1;
        Options options;
        u32 picture;
        u32 process_time;
        u32 delay;

        u32 unknown2;
        u32 range_count;

        u24 end_command;
    };
}

#endif