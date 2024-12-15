#pragma once

import wolf.command_placeholder;

namespace Loop {
    struct Command {
        u8 unknown1;
        u24 unknown2;

        CommandPlaceholder commands[9999];

        u64 end_command;
    };

    struct BreakCommand {
        u32 unknown1;
    };

    struct GotoStartCommand {
        u32 unknown1;
    };

    struct CountCommand {
        u8 unknown1;
        u32 count;

        u24 unknown2;

        CommandPlaceholder commands[9999];

        u64 end_command;
    };
}

namespace Transition {
    struct PrepareCommand {
        u32 unknown1;
    };

    struct ExecuteCommand {
        u32 unknown1;
    };

    struct SetCommand {
        u8 unknown1;

        u32 transition_number;
        u16 fade_frames;
        bool wait_until_done;

        u32 end_command;
    };
}

namespace MoveRoute {
    bitfield Options {
        bool repeat_actions: 1;
        bool skip_impossible_moves: 1;
        bool wait_until_done: 1;

        padding: 5;
    };

    enum Type: u16 {
        MoveDown            = 0x0000,
        MoveLeft            = 0x0001,
        MoveRight           = 0x0002,
        MoveUp              = 0x0003,
        MoveDownLeft        = 0x0004,
        MoveDownRight       = 0x0005,
        MoveUpLeft          = 0x0006,
        MoveUpRight         = 0x0007,

        FaceDown            = 0x0008,
        FaceLeft            = 0x0009,
        FaceRight           = 0x000A,
        FaceUp              = 0x000B,
        FaceDownLeft        = 0x000C,
        FaceDownRight       = 0x000D,
        FaceUpLeft          = 0x000E,
        FaceUpRight         = 0x000F,

        MoveRandom          = 0x0010,
        MoveTowardHero      = 0x0011,
        MoveAwayFromHero    = 0x0012,
        StepForward         = 0x0013,
        StepBackward        = 0x0014,

        TurnRight           = 0x0016,
        TurnLeft            = 0x0017,
        TurnLeftRightRandom = 0x0018,
        FaceRandomDirection = 0x0019,
        FaceTowardHero      = 0x001A,
        FaceAwayFromHero    = 0x001B,

        IdleAnimationOn     = 0x0020,
        IdleAnimationOff    = 0x0021,
        MoveAnimationOn     = 0x0022,
        MoveAnimationOff    = 0x0023,
        FixedDirectionOn    = 0x0024,
        FixedDirectionOff   = 0x0025,
        SlipThroughOn       = 0x0026,
        SlipThroughOff      = 0x0027,
        AlwaysOnTopOn       = 0x0028,
        AlwaysOnTopOff      = 0x0029,

        SetHalfTileMovement = 0x0030,
        SetFullTileMovement = 0x0031,

        SwitchToPattern1    = 0x0032,
        SwitchToPattern2    = 0x0033,
        SwitchToPattern3    = 0x0034,
        SwitchToPattern4    = 0x0038,
        SwitchToPattern5    = 0x0039,

        SetMoveSpeed        = 0x011D,
        SetMoveFrequency    = 0x011E,
        SetAnimationSpeed   = 0x011F,

        SetGraphic          = 0x012C,
        SetOpacity          = 0x012D,
        PlaySound           = 0x012E,
        WaitFrames          = 0x012F,

        ApproachEvent       = 0x0135,
        SetHeight           = 0x013A,

        Jump                = 0x0215,
        SetVariable         = 0x021C,

        ApproachPosition    = 0x0236,

        AddToVariable       = 0x0237,
    };

    struct Move {
        Type type;

        match(type) {
            (Type::ApproachEvent): {
                u32 event;
            }

            (Type::Jump): {
                u32 right;
                u32 down;
            }

            (Type::ApproachPosition): {
                u32 x;
                u32 y;
            }

            (Type::SetVariable | Type::AddToVariable): {
                u32 variable;
                u32 value;
            }

            (Type::SetMoveSpeed | Type::SetAnimationSpeed): {
                u32 speed;
            }

            (Type::SetMoveFrequency): {
                u32 frequency;
            }

            (Type::SetGraphic): {
                u32 graphic;
            }

            (Type::SetOpacity): {
                u32 opacity;
            }

            (Type::SetHeight): {
                u32 height;
            }

            (Type::PlaySound): {
                u32 sound;
            }

            (Type::WaitFrames): {
                u32 frame_count;
            }
        }

        u16 end_move;
    };

    struct Command {
        u8 unknown1;

        u32 target;

        u32 unknown2;
        u32 unknown3;
        Options options;

        u32 move_count;

        Move moves[move_count];
    };

    struct WaitForCommand {
        u32 unknown1;
    };
}

namespace MoveDuringEvents {
    struct OnCommand {
        u32 unknown1;
    };

    struct OffCommand {
        u32 unknown1;
    };
}

namespace Goto {
    struct TitleCommand {
        u32 unknown1;
    };

    struct GameEndCommand {
        u32 unknown1;
    };
}

namespace NonPictureUpdates {
    struct StopCommand {
        u32 unknown1;
    };

    struct ResumeCommand {
        u32 unknown1;
    };
}

namespace Event {
    struct ForceExitCommand {
        u32 unknown1;
    };

    struct EraseCommand {
        u8 unknown1;

        u32 event;
        u32 fade_frames;

        u24 end_command;
    };

    struct WaitCommand {
        u8 unknown1;
        u32 frame_count;
        u24 end_command;
    };
}

namespace Label {
    struct Command {
        u16 unknown1;
        u8 string_count;

        u32 label_length;
        char label[label_length];

        u8 end_command;
    };
}