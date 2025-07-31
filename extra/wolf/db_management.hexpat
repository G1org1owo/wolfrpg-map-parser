#ifndef DB_MANAGEMENT_HEXPAT
#define DB_MANAGEMENT_HEXPAT

namespace DBManagement {
    enum OperationType: u8 {
        Write = 0b00000000,
        Read  = 0b00000001,
    };

    enum DB: u8 {
        VarDB  = 0b00000000,
        SysDB  = 0b00000001,
        UserDB = 0b00000010
    };

    bitfield Options {
        DB db: 4;
        OperationType db_operation_type: 4;
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
    };

    bitfield Assignment {
        use_variable_as_reference: 1;
        padding: 3;
        AssignmentOperator operator: 4;
    };

    struct Command {
        u8 unknown1;
        u32 db_type;
        u32 data;
        u32 field;
        Assignment assignment;
        Options options;
        u16 unknown2;
        u32 value;
        u8 unknown3;
        u8 string_count;

        if(string_count > 0) {
            u32 value_length;
            char value_string[value_length];
        }

        if(string_count > 1) {
            u32 db_type_length;
            char db_type_name[db_type_length];
        }

        if (string_count > 2) {
            u32 data_length;
            char data_name[data_length];
        }

        if (string_count > 3) {
            u32 field_length;
            char field_name[field_length];
        }

        u8 end_command;
    };

    struct StringCommand {
        u8 unknown1;
        u32 db_type;
        u32 data;
        u32 field;
        Assignment assignment;
        Options options;
        u16 unknown2;
        u8 unknown3;
        u8 string_count;

        u32 value_length;
        char value[value_length];

        u32 db_type_length;
        char db_type_name[db_type_length];

        u32 data_length;
        char data_name[data_length];

        u32 field_length;
        char field_name[field_length];

        u8 end_command;
    };

    struct CSVCommand {
        u8 unknown1;
        u32 db_type;
        u32 data;
        u32 field;
        Assignment assignment;
        Options options;
        u16 unknown2;
        u32 entry_count;
        u8 unknown3;
        u8 string_count;

        u32 filename_length;;
        char filename[filename_length];

        u32 db_type_length;
        char db_type_name[db_type_length];

        u32 data_length;
        char data_name[data_length];

        u32 field_length;
        char field_name[field_length];

        u8 end_command;
    };
}

#endif