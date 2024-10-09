#ifndef PARTY_GRAPHICS_HEXPAT
#define PARTY_GRAPHICS_HEXPAT

namespace PartyGraphics {
    enum Operation: u8 {
        Remove          = 0x00,
        Insert          = 0x01,
        Replace         = 0x02,
        RemoveGraphic   = 0x03,
        Special         = 0x04,
    };

    enum SpecialOperation: u8 {
        PushCharactersToFront   = 0x00,
        EraseAllCharacters      = 0x01,
        WarpPartyToHero         = 0x02,
        StartHeroPartySynchro   = 0x03,
        CancelHeroPartySynchro  = 0x04,
        MakePartyTransparent    = 0x05,
        CancelPartyTransparency = 0x06,
        SavePartyMembers        = 0x07,
        LoadPartyMembers        = 0x08,
        TurnOnPartyFollowing    = 0x09,
        TurnOffPartyFollowing   = 0x0a,
    };

    bitfield Options {
        Operation operation: 4;
        SpecialOperation special_operation: 4;
        bool is_graphics_variable: 8;

        padding: 16;
    };

    struct Command {
        u8 unknown1;
        Options options;

        match(options.operation) {
            (Operation::Remove | Operation::Insert | Operation::Replace): u32 member;
        }

        if(options.is_graphics_variable) {
            u32 graphics;
        }

        u8 end_options;
        bool is_graphics_string;

        if(is_graphics_string) {
            u32 filename_length;
            char filename[filename_length];
        }

        u8 end_command;
    };
}

#endif