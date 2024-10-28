#ifndef SAVE_LOAD_HEXPAT
#define SAVE_LOAD_HEXPAT

namespace SaveLoad {
    enum Operation: u32 {
        Save = 0x00000000,
        Load = 0x00000001,
    };

    struct Command {
        u8 unknown1;
        Operation operation;
        u32 save_number;

        u24 end_command;
    };

    struct LoadVariableCommand {
        u8 unknown1;
        u32 target_variable;
        u32 save_number;
        u32 source_variable;
        u32 target_is_pointer;

        u24 end_command;
    };

    struct SaveVariableCommand {
        u8 unknown1;
        u32 source_variable;
        u32 save_number;
        u32 target_variable;
        u32 source_is_pointer;

        u24 end_command;
    };
}

#endif