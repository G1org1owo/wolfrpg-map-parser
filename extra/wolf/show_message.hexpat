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

    struct CommentCommand {
        u24 unknown1;
        u32 comment_length;
        char comment[comment_length];
        u8 end_command;
    };

    struct DebugTextCommand {
        u24 unknown1;
        u32 text_length;
        char text[text_length];
        u8 end_command;
    };

    struct ForceCloseMessageCommand {
        u32 unknown1;
    };

    struct ClearDebugTextCommand {
        u32 unknown1;
    };
}

#endif