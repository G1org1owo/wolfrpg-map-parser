#pragma magic [ 00 00 00 00 00 00 00 00 00 00 57 4F 4C 46 4D 00 ] @ 0x00

import wolf.show_message;
import wolf.show_choice;
import wolf.set_variable;
import wolf.db_management;
import wolf.set_string;
import wolf.set_variable_plus;
import wolf.condition_number;
import wolf.condition_string;
import wolf.input_key;
import wolf.picture;
import wolf.effect;
import wolf.sound;
import wolf.save_load;
import wolf.party_graphics;
import wolf.chip_management;
import wolf.transfer;



struct ExitCommand {
    u32 unknown1;
};

enum CommandSignature: u32 {
    ShowMessage                 = 0x01650000,
    ShowChoice                  = 0x02660000,
    SetVariable                 = 0x05790000,
    SetVariableRange            = 0x06790000,
    DBManagement                = 0x06fa0000,
    DBManagementString          = 0x05fa0000,
    DBManagementCSV             = 0x06fb0000,
    SetString                   = 0x037a0000,
    SetStringDynamic            = 0x047a0000,
    SetVariablePlus             = 0x057C0000,
    SetVariablePlusOther        = 0x047C0000,
    ConditionNumber             = 0x056f0000,
    DoubleConditionNumber       = 0x086f0000,
    TripleConditionNumber       = 0x0b6f0000,
    
    ConditionString1            = 0x03700000,
    ConditionString2            = 0x04700000,
    ConditionString3            = 0x05700000,
    ConditionString4            = 0x06700000,
    ConditionString5            = 0x07700000,
    ConditionString6            = 0x08700000,
    ConditionString7            = 0x09700000,
    ConditionString8            = 0x0a700000,
    
    InputKey1                   = 0x037B0000,
    InputKey2                   = 0x047B0000,
    AutomaticInput1             = 0x027D0000,
    AutomaticInput2             = 0x037D0000,
    AutomaticInput3             = 0x047D0000,
    InputToggle1                = 0x027E0000,
    InputToggle2                = 0x037E0000,
    
    PictureShow1                = 0x0C960000,
    PictureShow2                = 0x1A960000,
    PictureShow3                = 0x13960000,
    PictureShow4                = 0x14960000,
    PictureShow5                = 0x0E960000,
    PictureErase1               = 0x04960000,
    PictureErase2               = 0x07960000,
    PictureErase3               = 0x05960000,
    PictureErase4               = 0x03960000,
    
    EffectBase                  = 0x08220100,
    EffectMapShake              = 0x03180100,
    EffectScrollScreen          = 0x04190100,
    EffectChangeColor           = 0x03970000,
    
    SoundFilename1              = 0x088C0000,
    SoundFilename2              = 0x078C0000,
    SoundVariable               = 0x058C0000,
    SoundVariableMin1           = 0x028C0000,
    SoundVariableMin2           = 0x048C0000,
    
    SaveLoad                    = 0x03DC0000,
    LoadVariable                = 0x05DD0000,
    SaveVariable                = 0x05DE0000,
    
    PartyGraphics1              = 0x030E0100,
    PartyGraphics2              = 0x040E0100,
    PartyGraphics3              = 0x020E0100,

    MapChipSettings             = 0x03F00000,
    MapSwitchChipset            = 0x02F10000,
    MapOverwriteChips           = 0x07F20000,

    Transfer                    = 0x06820000,

    Exit                        = 0x01000000,
};

struct Command {
    be CommandSignature signature;
    
    match (signature) {
        (CommandSignature::ShowMessage): ShowMessage::Command command;
        (CommandSignature::ShowChoice): ShowChoice::Command command;
        
        (CommandSignature::SetVariable): SetVariable::Command command;
        (CommandSignature::SetVariableRange): SetVariable::RangeCommand command;
        
        (CommandSignature::DBManagement): DBManagement::Command command;
        (CommandSignature::DBManagementString): DBManagement::StringCommand command;
        (CommandSignature::DBManagementCSV): DBManagement::CSVCommand command;

        (CommandSignature::SetString): SetString::Command command;
        (CommandSignature::SetStringDynamic): SetString::DynamicCommand command;

        (CommandSignature::SetVariablePlus): SetVariablePlus::Command command;
        (CommandSignature::SetVariablePlusOther): SetVariablePlus::OtherCommand command;

        (CommandSignature::ConditionNumber): ConditionNumber::Command command;
        (CommandSignature::DoubleConditionNumber): ConditionNumber::DoubleCommand command;
        (CommandSignature::TripleConditionNumber): ConditionNumber::TripleCommand command;
        
        (CommandSignature::ConditionString1 | CommandSignature::ConditionString2 |
         CommandSignature::ConditionString3 | CommandSignature::ConditionString4 |
         CommandSignature::ConditionString5 | CommandSignature::ConditionString6 |
         CommandSignature::ConditionString7 | CommandSignature::ConditionString8): {
            u32 total_count = signature;
            ConditionString::Command<total_count> command;
        }
        
        (CommandSignature::InputKey1 | CommandSignature::InputKey2): InputKey::Command command;
        
        (CommandSignature::AutomaticInput1 | CommandSignature::AutomaticInput2 |
         CommandSignature::AutomaticInput3): AutomaticInput::Command command;
         
        (CommandSignature::InputToggle1 | CommandSignature::InputToggle2): InputToggle::Command command;
        
        (CommandSignature::PictureShow1): Picture::ShowCommand command;
        (CommandSignature::PictureShow2 | CommandSignature::PictureShow3 |
         CommandSignature::PictureShow4 | CommandSignature::PictureShow5): Picture::ShowCompleteCommand command;
         
        (CommandSignature::PictureErase1 | CommandSignature::PictureErase4): Picture::EraseCommand command;
        (CommandSignature::PictureErase2 | CommandSignature::PictureErase3): Picture::EraseCompleteCommand command;

        (CommandSignature::EffectBase): Effect::Command command;
        (CommandSignature::EffectMapShake): EffectMapShake::Command command;
        (CommandSignature::EffectScrollScreen): EffectScrollScreen::Command command;
        (CommandSignature::EffectChangeColor): EffectChangeColor::Command command;

        (CommandSignature::SoundFilename1 | CommandSignature::SoundFilename2): Sound::FilenameCommand command;
        (CommandSignature::SoundVariable): Sound::VariableCommand command;
        (CommandSignature::SoundVariableMin1 | CommandSignature::SoundVariableMin2): Sound::VariableMinCommand command;

        (CommandSignature::SaveLoad): SaveLoad::Command command;
        (CommandSignature::LoadVariable): SaveLoad::LoadVariableCommand command;
        (CommandSignature::SaveVariable): SaveLoad::SaveVariableCommand command;

        (CommandSignature::PartyGraphics1 | CommandSignature::PartyGraphics2 |
         CommandSignature::PartyGraphics3): PartyGraphics::Command command;

        (CommandSignature::MapChipSettings): MapChip::SettingsCommand command;
        (CommandSignature::MapSwitchChipset): MapChip::SwitchChipsetCommand command;
        (CommandSignature::MapOverwriteChips): MapChip::OverwriteChipsCommand command;

        (CommandSignature::Transfer): Transfer::Command command;

        (CommandSignature::Exit): {
            ExitCommand command;
            break;
        }
    }
};

enum EventTrigger: u8 {
    ConfirmKey      = 0x00,
    AutoRun         = 0x01,
    ParallelProcess = 0x02,
    PlayerTouch     = 0x03,
    EventTouch      = 0x04
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