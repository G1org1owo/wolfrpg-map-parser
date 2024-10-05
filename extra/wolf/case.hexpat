#ifndef CASE_HEXPAT
#define CASE_HEXPAT

using Command;

namespace Case {
    enum CaseType: u32 {
        Choice  = 0x02910100,
        Extra   = 0x02920100,
        Cancel  = 0x02a50100,
        Else    = 0x02a40100,
    };

    struct Case {
        be CaseType case_type;
        u8 unknown1;
        u8 case_id;
        u16 unknown2;
        u32 unknown3;
        Command commands[9999];
    };
}

#endif