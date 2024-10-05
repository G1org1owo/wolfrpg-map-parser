#ifndef SHOW_MESSAGE_HEXPAT
#define SHOW_MESSAGE_HEXPAT

namespace ShowMessage {
    struct Command {
        u8 unknown1;
        u8 unknown2;
        u8 unknown3;
        u32 message_length;
        char message[message_length];
        u8 command_end;
    };
}

#endif