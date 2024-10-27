#ifndef SOUND_HEXPAT
#define SOUND_HEXPAT

namespace Sound {
    enum ProcessType: u8 {
        Playback    = 0x00,
        Preload     = 0x01,
        FreeMemory  = 0x03,
    };

    enum Operation: u8 {
        SetBGM  = 0x00,
        SetBGS  = 0x01,
        SetSE   = 0x02,
    };

    bitfield Options {
        ProcessType process_type: 4;
        Operation operation: 4;
    };

    enum SoundType: u8 {
        DBEntry     = 0x00,
        Variable    = 0x01,
        Filename    = 0x02,
    };

    struct FilenameCommand {
        u8 unknown1;
        Options options;
        u16 systemdb_entry;
        SoundType sound_type;

        if(options.operation == Operation::SetSE) {
            u32 delay_playback;
        } else {
            u32 fade_time;
        }

        u32 variable;
        u32 start_time;
        u32 volume;
        u32 tempo;

        if(options.operation != Operation::SetSE) {
            u32 loop_point;
        }

        u8 end_options;
        bool is_sound_string;

        if(is_sound_string) {
            u32 sound_length;
            char sound[sound_length];
        }

        u8 end_command;
    };

    struct VariableCommand {
        u8 unknown1;
        Options options;
        u16 systemdb_entry;
        SoundType sound_type;

        if(options.operation == Operation::SetSE) {
            u32 delay_playback;
        } else {
            u32 fade_time;
        }

        u32 variable;
        u32 start_time;

        u24 end_command;
    };

    struct VariableMinCommand {
        u8 unknown1;
        Options options;
        u16 systemdb_entry;
        SoundType sound_type;

        if(sound_type == SoundType::Variable) {
            u32 unknown2;
            u32 variable;
        }

        u24 end_command;
    };
}

#endif