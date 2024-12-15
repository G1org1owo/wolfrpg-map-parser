#ifndef COMMON_EVENT_HEXPAT
#define COMMON_EVENT_HEXPAT

namespace CommonEvent {
    bitfield Arguments {
        number_arguments: 4;
        string_arguments: 4;
    };

    bitfield Options {
        padding: 4;
        bool is_arg1_string: 1;
        bool is_arg2_string: 1;
        bool is_arg3_string: 1;
        bool is_arg4_string: 1;
        padding: 8;
        bool has_return_value: 1;
        padding: 7;
    };

    struct String {
        u32 string_length;
        char string[string_length];
    };

    struct CallEventCommand {
        u8 unknown1;
        u32 target;
        Arguments argument_count;
        Options options;

        u32 arguments[argument_count.number_arguments];
        u32 string_arguments_variables[argument_count.string_arguments];

        if(options.has_return_value) {
            u32 return_variable;
        }

        u8 unknown2;
        u8 string_count;

        if(string_count > 0) {
            String variable_name;
            String strings_arguments[string_count-1];
        }

        u8 end_command;
    };

    struct CallEventByVariableCommand {
        u8 unknown1;
        u32 target;
        u32 page;

        u24 end_command;
    };

    struct ReserveEventCommand {
        u8 unknown1;
        u32 target;
        u32 unknown2;

        u24 end_command;
    };
}

#endif