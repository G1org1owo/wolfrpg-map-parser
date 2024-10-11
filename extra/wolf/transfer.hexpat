#ifndef TRANSFER_HEXPAT
#define TRANSFER_HEXPAT

namespace Transfer {
    enum Transition: u8 {
        None    = 0x00,
        NoFade  = 0x01,
        Fade    = 0x02,
    };

    bitfield Options {
        bool precise_coordinates: 1;
        padding: 3;
        Transition transition: 4;

        padding: 24;
    };

    struct Command {
        u8 unknown1;
        u32 target;

        if(target == 0xFFFFD8EF) {
            u32 db_variable;
            u32 destination_x;
            u32 destination_y;
        } else {
            u32 destination_x;
            u32 destination_y;
            u32 destination_map;
        }

        Options options;

        u24 end_command;
    };
}

#endif