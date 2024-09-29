#pragma magic [ 00 00 00 00 00 00 00 00 00 00 57 4F 4C 46 4D 00 ] @ 0x00

using Command;

struct ShowMessageCommand {
    u8 unknown1;
    u8 unknown2;
    u8 unknown3;
    u32 message_length;
    char message[message_length];
    u8 command_end;
};

bitfield SelectedChoices {
    cancel: 4;
    choice: 4;
};

bitfield ExtraCases {
    left_key: 1;
    right_key: 1;
    force_exit: 1;
    padding: 5;
};

fn extraCasesCount(ExtraCases extra_cases) {
    return extra_cases.left_key + 
        extra_cases.right_key + 
        extra_cases.force_exit;
};

struct Choice {
    u32 choice_length;
    char choice[choice_length];
};

enum CaseType: u32 {
    Choice = 0x02910100,
    Extra = 0x02920100,
    Cancel = 0x02a50100
};

struct Case {
    be CaseType case_type;
    u8 unknown1;
    u8 case_id;
    u16 unknown2;
    u32 unknown3;
    Command commands[9999];
};

fn calculateCaseCount(u8 choice_count, ExtraCases extra_cases, SelectedChoices selected_choices) {
    return choice_count +
        extraCasesCount(extra_cases) + 
        (selected_choices.cancel == 0? 1 : 0);
};

struct ShowChoiceCommand {
    u8 unknown1;
    SelectedChoices selected_choices;
    ExtraCases extra_cases;
    u16 unknown2;
    u8 unknown3;
    u8 choice_count;
    Choice choices[choice_count];
    u8 choices_end;
    Case cases[calculateCaseCount(choice_count, extra_cases, selected_choices)];
    u32 unknown4;
    u32 unknown5;
};

bitfield SetVariableOptions {
    bound_result: 1;
    real_number_calculation: 1;
    left_not_variable: 1;
    right_not_variable: 1;
    use_variable_as_reference: 1;
    use_left_as_reference: 1;
    use_right_as_reference: 1;
    padding: 1;
};

enum SetVariableAssignment: u8 {
    Equals = 0x00,
    PlusEquals = 0x01,
    MinusEquals = 0x02,
    TimesEquals = 0x03,
    DivideEquals = 0x04,
    RemainderEquals = 0x05,
    LowerBound = 0x06,
    UpperBound = 0x07,
    Absolute = 0x08,
    Angle = 0x09,
    Sin = 0x0a,
    Cos = 0x0b,
};

enum SetVariableCalculation: u8 {
    Plus = 0x00,
    Minus = 0x01,
    Times = 0x02,
    Divides = 0x03,
    Remainder = 0x04,
    BitwiseAnd = 0x05,
    BitwiseNot = 0x06,
};

bitfield SetVariableOperators {
    SetVariableAssignment assignment: 4;
    SetVariableCalculation calculation: 4;
};

struct SetVariableCommand {
    u8 unknown1;
    u32 variable;
    u32 left_side;
    u32 right_side;
    SetVariableOptions options;
    SetVariableOperators operators;
    u32 unknown2;
    u8 end_command;
};

struct SetVariableRangeCommand {
    u8 unknown1;
    u32 variable;
    u32 left_side;
    u32 right_side;
    SetVariableOptions options;
    SetVariableOperators operators;
    u16 range_start;
    u16 range_end;
    u32 unknown2;
    u8 end_command;
};

enum DBOperationType: u8 {
    Write = 0b00000000,
    Read  = 0b00000001,
};

enum DBType: u8 {
    VarDB  = 0b00000000,
    SysDB  = 0b00000001,
    UserDB = 0b00000010
};

bitfield DBManagementOptions {
    DBType db_type: 4;
    DBOperationType db_operation_type: 4;
};

bitfield DBManagementAssignment {
    use_variable_as_reference: 1;
    padding: 3;
    SetVariableAssignment assignment: 4;
};

struct DBManagementCommand {
    u8 unknown1;
    u32 db_type;
    u32 data;
    u32 field;
    DBManagementAssignment assignment;
    DBManagementOptions options;
    u16 unknown2;
    u32 value;
    u8 unknown3;
    u8 string_count;
    u32 value_length;
    char value_string[value_length];
    u32 db_type_length;
    char db_type_name[db_type_length];
    u32 data_length;
    char data_name[data_length];
    u32 field_length;
    char field_name[field_length];
    u8 end_command;
};

struct DBManagementStringCommand {
    u8 unknown1;
    u32 db_type;
    u32 data;
    u32 field;
    DBManagementAssignment assignment;
    DBManagementOptions options;
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

struct DBManagementCSVCommand {
    u8 unknown1;
    u32 db_type;
    u32 data;
    u32 field;
    DBManagementAssignment assignment;
    DBManagementOptions options;
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

struct ExitCommand {
    u32 unknown1;
};

enum CommandSignature: u32 {
    ShowMessage         = 0x01650000,
    ShowChoice          = 0x02660000,
    SetVariable         = 0x05790000,
    SetVariableRange    = 0x06790000,
    DBManagement        = 0x06fa0000,
    DBManagementString  = 0x05fa0000,
    DBManagementCSV     = 0x06fb0000,
    Exit                = 0x01000000,
};

struct Command {
    be CommandSignature signature;
    
    match (signature) {
        (CommandSignature::ShowMessage): ShowMessageCommand command;
        (CommandSignature::ShowChoice): ShowChoiceCommand command;
        (CommandSignature::SetVariable): SetVariableCommand command;
        (CommandSignature::SetVariableRange): SetVariableRangeCommand command;
        (CommandSignature::DBManagement): DBManagementCommand command;
        (CommandSignature::DBManagementString): DBManagementStringCommand command;
        (CommandSignature::DBManagementCSV): DBManagementCSVCommand command;
        (CommandSignature::Exit): {
            ExitCommand command;
            break;
        }
    }
};

enum EventTrigger: u8 {
    ConfirmKey = 0x00,
    AutoRun = 0x01,
    ParallelProcess = 0x02,
    PlayerTouch = 0x03,
    EventTouch = 0x04
};

bitfield EventOptions {
    idle_animation: 1;
    move_animation: 1;
    fixed_direction: 1;
    slip_through: 1;
    above_hero: 1;
    square_hitbox: 1;
    half_step_up: 1;
    padding: 1;
};

struct Event {
    u8 signature[5];
    u32 id;
    u32 title_length;
    char title[title_length];
    u32 position_x;
    u32 position_y;
    u32 page_count;
    u32 unknown1;
    u8 icon_signature[5];
    u32 icon_length;
    char icon_name[icon_length];
    u8 icon_row;
    u8 icon_column;
    u8 icon_opacity;
    u8 icon_blend;
    EventTrigger event_trigger;
    u8 condition1_operator;
    u8 condition2_operator;
    u8 condition3_operator;
    u8 condition4_operator;
    u32 condition1_variable;
    u32 condition2_variable;
    u32 condition3_variable;
    u32 condition4_variable;
    u32 condition1_value;
    u32 condition2_value;
    u32 condition3_value;
    u32 condition4_value;
    u8 animation_speed;
    u8 move_speed;
    u8 move_frequency;
    u8 move_route;
    EventOptions options;
    u8 unknown2;
    u32 unknown3;
    u32 command_count;
    Command commands[command_count];
    u32 unknown4;
    u8 shadow_graphic;
    u8 range_extension_x;
    u8 range_extension_y;
    u8 page_end;
    u8 event_end;
};

struct Map {
    u8 signature[25];
    u32 offset;
    u8 unknown[offset];
    u32 tileset;
    u32 width;
    u32 height;
    u32 event_count;
    u32 layer1[width*height];
    u32 layer2[width*height];
    u32 layer3[width*height];
    Event events[event_count];
    u8 end_signature;
};

Map map @ 0x00;