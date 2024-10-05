#ifndef SET_STRING_HEXPAT
#define SET_STRING_HEXPAT

namespace SetString {
    enum ContentType: u8 {
        StringLiteral   = 0b00000000,
        StringVariable  = 0b00000001,
        Variable        = 0b00000010,
        UserInput       = 0b00000011,
    };

    enum VariableType: u8 {
        StringVariable      = 0b00000000,
        VariableReference   = 0b00000001,
    };

    bitfield Options {
        ContentType content_type: 4;
        VariableType variable_type: 4;
    };

    enum StringOperation: u8 {
        Equals              = 0x00,
        PlusEquals          = 0x01,
        CopyFirstLine       = 0x02,
        CutFirstLine        = 0x03,
        CutFirstCharacter   = 0x04,
        LoadFileContents    = 0x05,
        ExportToFile        = 0x06,
        GetFileList         = 0x07,
        RemoveInstancesOf   = 0x08,
        Replace             = 0x09,
        CutUpTo             = 0x0a,
        CutAfter            = 0x0b,
    };

    bitfield Operation {
        StringOperation operation: 4;
        cancel: 1;
        replace: 1;
        padding: 2;
    };

    struct Command {
        u8 unknown1;
        u32 variable;
        Options options;
        Operation operation;
        u16 unknown2;
        u8 unknown3;
        u8 string_count;
        u32 string_length;
        char string[string_length];

        if(string_count == 2) {
            u32 replace_length;
            char replace[replace_length];
        }

        u8 command_end;
    };

    struct DynamicCommand {
        u8 unknown1;
        u32 variable;
        Options options;
        Operation operation;
        u16 unknown2;

        match(options.content_type) {
            (ContentType::UserInput): u32 input_length;
            (_): u32 source;
        }

        u16 unknown3;
        u8 command_end;
    };
}

#endif