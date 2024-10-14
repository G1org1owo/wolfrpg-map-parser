#ifndef SHOW_CHOICE_HEXPAT
#define SHOW_CHOICE_HEXPAT

import wolf.case;

namespace ShowChoice {
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

    struct Choice {
        u32 choice_length;
        char choice[choice_length];
    };

    fn calculateCaseCount(u8 choice_count, ExtraCases extra_cases, SelectedChoices selected_choices) {
        return choice_count +
            ShowChoice::extraCasesCount(extra_cases) +
            (selected_choices.cancel == 0? 1 : 0);
    };

    fn extraCasesCount(ExtraCases extra_cases) {
        return extra_cases.left_key +
            extra_cases.right_key +
            extra_cases.force_exit;
    };

    struct Command {
        u8 unknown1;
        SelectedChoices selected_choices;
        ExtraCases extra_cases;
        u16 unknown2;
        u8 unknown3;
        u8 choice_count;
        Choice choices[choice_count];
        u8 choices_end;
        Case::Case cases[ShowChoice::calculateCaseCount(choice_count, extra_cases, selected_choices)];
        u32 unknown4;
        u32 unknown5;
    };
}

#endif