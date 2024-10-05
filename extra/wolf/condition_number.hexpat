#ifndef CONDITION_NUMBER_HEXPAT
#define CONDITION_NUMBER_HEXPAT

import wolf.case;

namespace ConditionNumber {
    bitfield CaseCount {
        case_count: 4;
        else_case: 1;
        padding: 11;
    };

    enum CompareOperator: u8 {
        GreaterThan     = 0x00,
        GreaterOrEquals = 0x01,
        Equals          = 0x02,
        LessOrEquals    = 0x03,
        LessThan        = 0x04,
        NotEqual        = 0x05,
        BitwiseAnd      = 0x06,
    };

    bitfield Operator {
        CompareOperator operator: 4;
        not_variable: 1;
        padding: 3;
    };

    struct Condition {
        u16 unknown1;
        u32 variable;
        u32 value;
        Operator operator;
        u8 unknown2;
    };

    struct Command {
        u8 unknown1;
        CaseCount case_count;
        Condition condition;
        u32 unknown3;
        u8 unknown4;

        Case::Case case;

        if(case_count.else_case == 1) {
            Case::Case else_case;
        }

        u8 end_command[8];
    };

    struct DoubleCommand {
        u8 unknown1;
        CaseCount case_count;
        Condition condition1;
        Condition condition2;
        u32 unknown2;
        u8 unknown3;

        Case::Case case1;
        Case::Case case2;

        if(case_count.else_case == 1) {
            Case::Case else_case;
        }

        u8 end_command[8];
    };

    struct TripleCommand {
        u8 unknown1;
        CaseCount case_count;
        Condition condition1;
        Condition condition2;
        Condition condition3;
        u32 unknown2;
        u8 unknown3;

        Case::Case case1;
        Case::Case case2;
        Case::Case case3;

        if(case_count.else_case == 1) {
            Case::Case else_case;
        }

        u8 end_command[8];
    };
}

#endif