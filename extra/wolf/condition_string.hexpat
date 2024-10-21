#ifndef CONDITION_STRING_HEXPAT
#define CONDITION_STRING_HEXPAT

import wolf.case;

namespace ConditionString {
    enum CompareOperator: u8 {
        Equals      = 0x00,
        NotEquals   = 0x01,
        Includes    = 0x02,
        StartsWith  = 0x03,
    };

    bitfield Operator {
        is_value_variable: 1;
        padding: 3;
        CompareOperator operator: 4;
    };

    struct Variable {
        u24 variable;
        Operator operator;
    };

    struct Condition {
        u32 length;
        char string[length];
    };

    fn valueCount(u32 signature, u32 case_count) {
        u32 val = signature;
        val = val >> 24 & 0xff;
        return val - 2 - case_count;
    };

    bitfield CaseCount {
        case_count: 4;
        else_case: 1;
        padding: 3;

        padding: 24;
    };

    struct Command<auto total_count> {
        u8 unknown1;
        CaseCount case_count;

        Variable variables[case_count.case_count];

        u32 values[ConditionString::valueCount(total_count, case_count.case_count)];

        u8 values_end;
        u8 condition_count;

        Condition conditions[condition_count];
        u8 condition_end;

        Case::Case cases[case_count.case_count];

        if(case_count.else_case == 1) {
            Case::Case else_case;
        }

        u8 end_command[8];
    };
}

#endif