pub enum Signature {
    ShowMessage                     = 0x01650000,
    Comment                         = 0x01670000,
    DebugText                       = 0x016A0000,
    ForceCloseMessage               = 0x01690000,
    ClearDebugText                  = 0x016B0000,
    ShowChoice                      = 0x02660000,
    SetVariableBase                 = 0x05790000,
    SetVariableRange                = 0x06790000,
    DBManagementBase                = 0x06fa0000,
    DBManagementString              = 0x05fa0000,
    DBManagementCsv                 = 0x06fb0000,
    SetStringBase                   = 0x037a0000,
    SetStringDynamic                = 0x047a0000,
    SetVariablePlusBase             = 0x057c0000,
    SetVariablePlusOther            = 0x047c0000,
    NumberCondition                 = 0x056f0000,
    NumberConditionDouble           = 0x086f0000,
    NumberConditionTriple           = 0x0b6f0000,
    StringCondition                 = 0x03700000,
    StringConditionTwo              = 0x04700000,
    StringConditionThree            = 0x05700000,
    StringConditionFour             = 0x06700000,
    StringConditionFive             = 0x07700000,
    StringConditionSix              = 0x08700000,
    StringConditionSeven            = 0x09700000,
    StringConditionEight            = 0x0a700000,
    InputKeyBase                    = 0x037b0000,
    InputKeyKeyboardOrPad           = 0x047b0000,
    AutomaticInputBasic             = 0x027d0000,
    AutomaticInputKeyboard          = 0x037d0000,
    AutomaticInputMouse             = 0x047d0000,
    InputToggleBasic                = 0x027e0000,
    InputToggleDevice               = 0x037e0000,
    PictureShowBase                 = 0x0c960000,
    PictureShowBaseByVar            = 0x0d960000,
    PictureShowColors               = 0x0e960000,
    PictureShowDelay                = 0x0f960000,
    PictureShowRange                = 0x10960000,
    PictureShowColorValues          = 0x13960000,
    PictureShowZoom                 = 0x14960000,
    PictureShowFreeTransform        = 0x1a960000,
    PictureEraseDelayReset          = 0x03960000,
    PictureEraseBase                = 0x04960000,
    PictureEraseDelay               = 0x05960000,
    PictureEraseRange               = 0x07960000,
    EffectBase                      = 0x08220100,
    EffectMapShake                  = 0x03180100,
    EffectScrollScreen              = 0x04190100,
    EffectChangeColor               = 0x03970000,
    SoundFilename                   = 0x088c0000,
    SoundFilenameSe                 = 0x078c0000,
    SoundVariable                   = 0x058c0000,
    SoundFreeAll                    = 0x028c0000,
    SoundFreeAllVariable            = 0x048c0000,
    SaveLoadBase                    = 0x03dc0000,
    LoadVariable                    = 0x05dd0000,
    SaveVariable                    = 0x05de0000,
    PartyGraphicsBase               = 0x030e0100,
    PartyGraphicsVariable           = 0x040e0100,
    PartyGraphicsNoMember           = 0x020e0100,
    ChipManagementSettings          = 0x03f00000,
    ChipManagementSwitchSet         = 0x02f10000,
    ChipManagementOverwrite         = 0x07f20000,
    Transfer                        = 0x06820000,
    Loop                            = 0x01aa0000,
    BreakLoop                       = 0x01ab0000,
    GotoLoopStart                   = 0x01b00000,
    PrepareTransition               = 0x01a10000,
    ExecuteTransition               = 0x01a20000,
    SetTransition                   = 0x03a00000,
    Move                            = 0x02c90000,
    WaitForMove                     = 0x01ca0000,
    MoveDuringEventsOn              = 0x01e60000,
    MoveDuringEventsOff             = 0x01e70000,
    GotoTitle                       = 0x01ae0000,
    GameEnd                         = 0x01af0000,
    StopNonPictureGraphicUpdates    = 0x01b10000,
    ResumeNonPictureGraphicUpdates  = 0x01b20000,
    ForceExitEvent                  = 0x01ac0000,
    EraseEvent                      = 0x03ad0000,
    Wait                            = 0x02b40000,
    LoopCount                       = 0x02b30000,
    LabelPoint                      = 0x01d40000,
    LabelJump                       = 0x01d50000,
    CallEvent1                      = 0x06d20000,
    CallEvent2                      = 0x05D20000,
    CallEvent3                      = 0x07D20000,
    CallEventByName1                = 0x062C0100,
    CallEventByName2                = 0x052C0100,
    CallEventByName3                = 0x0B2C0100,
    CallEventByName4                = 0x032c0100,
    CallEventByName5                = 0x082c0100,
    CallEventByName6                = 0x042c0100,
    CallEventByVariable             = 0x03D20000,
    ReserveEvent                    = 0x03D30000,
    Exit                            = 0x01000000,
    Unknown
}

impl Signature {
    pub const fn new(signature: u32) -> Self {
        match signature {
            0x01650000 => Self::ShowMessage,
            0x01670000 => Self::Comment,
            0x016A0000 => Self::DebugText,
            0x01690000 => Self::ForceCloseMessage,
            0x016B0000 => Self::ClearDebugText,
            0x02660000 => Self::ShowChoice,
            0x05790000 => Self::SetVariableBase,
            0x06790000 => Self::SetVariableRange,
            0x06fa0000 => Self::DBManagementBase,
            0x05fa0000 => Self::DBManagementString,
            0x06fb0000 => Self::DBManagementCsv,
            0x037a0000 => Self::SetStringBase,
            0x047a0000 => Self::SetStringDynamic,
            0x057c0000 => Self::SetVariablePlusBase,
            0x047c0000 => Self::SetVariablePlusOther,
            0x056f0000 => Self::NumberCondition,
            0x086f0000 => Self::NumberConditionDouble,
            0x0b6f0000 => Self::NumberConditionTriple,
            0x03700000 => Self::StringCondition,
            0x04700000 => Self::StringConditionTwo,
            0x05700000 => Self::StringConditionThree,
            0x06700000 => Self::StringConditionFour,
            0x07700000 => Self::StringConditionFive,
            0x08700000 => Self::StringConditionSix,
            0x09700000 => Self::StringConditionSeven,
            0x0a700000 => Self::StringConditionEight,
            0x037b0000 => Self::InputKeyBase,
            0x047b0000 => Self::InputKeyKeyboardOrPad,
            0x027d0000 => Self::AutomaticInputBasic,
            0x037d0000 => Self::AutomaticInputKeyboard,
            0x047d0000 => Self::AutomaticInputMouse,
            0x027e0000 => Self::InputToggleBasic,
            0x037e0000 => Self::InputToggleDevice,
            0x0c960000 => Self::PictureShowBase,
            0x0d960000 => Self::PictureShowBaseByVar,
            0x0e960000 => Self::PictureShowColors,
            0x0f960000 => Self::PictureShowDelay,
            0x10960000 => Self::PictureShowRange,
            0x13960000 => Self::PictureShowColorValues,
            0x14960000 => Self::PictureShowZoom,
            0x1a960000 => Self::PictureShowFreeTransform,
            0x03960000 => Self::PictureEraseDelayReset,
            0x04960000 => Self::PictureEraseBase,
            0x05960000 => Self::PictureEraseDelay,
            0x07960000 => Self::PictureEraseRange,
            0x08220100 => Self::EffectBase,
            0x03180100 => Self::EffectMapShake,
            0x04190100 => Self::EffectScrollScreen,
            0x03970000 => Self::EffectChangeColor,
            0x088c0000 => Self::SoundFilename,
            0x078c0000 => Self::SoundFilenameSe,
            0x058c0000 => Self::SoundVariable,
            0x028c0000 => Self::SoundFreeAll,
            0x048c0000 => Self::SoundFreeAllVariable,
            0x03dc0000 => Self::SaveLoadBase,
            0x05dd0000 => Self::LoadVariable,
            0x05de0000 => Self::SaveVariable,
            0x030e0100 => Self::PartyGraphicsBase,
            0x040e0100 => Self::PartyGraphicsVariable,
            0x020e0100 => Self::PartyGraphicsNoMember,
            0x03f00000 => Self::ChipManagementSettings,
            0x02f10000 => Self::ChipManagementSwitchSet,
            0x07f20000 => Self::ChipManagementOverwrite,
            0x06820000 => Self::Transfer,
            0x01aa0000 => Self::Loop,
            0x01ab0000 => Self::BreakLoop,
            0x01b00000 => Self::GotoLoopStart,
            0x01a10000 => Self::PrepareTransition,
            0x01a20000 => Self::ExecuteTransition,
            0x03a00000 => Self::SetTransition,
            0x02c90000 => Self::Move,
            0x01ca0000 => Self::WaitForMove,
            0x01e60000 => Self::MoveDuringEventsOn,
            0x01e70000 => Self::MoveDuringEventsOff,
            0x01ae0000 => Self::GotoTitle,
            0x01af0000 => Self::GameEnd,
            0x01b10000 => Self::StopNonPictureGraphicUpdates,
            0x01b20000 => Self::ResumeNonPictureGraphicUpdates,
            0x01ac0000 => Self::ForceExitEvent,
            0x03ad0000 => Self::EraseEvent,
            0x02b40000 => Self::Wait,
            0x02b30000 => Self::LoopCount,
            0x01d40000 => Self::LabelPoint,
            0x01d50000 => Self::LabelJump,
            0x06d20000 => Self::CallEvent1,
            0x05D20000 => Self::CallEvent2,
            0x07D20000 => Self::CallEvent3,
            0x062C0100 => Self::CallEventByName1,
            0x052C0100 => Self::CallEventByName2,
            0x0B2C0100 => Self::CallEventByName3,
            0x032c0100 => Self::CallEventByName4,
            0x082c0100 => Self::CallEventByName5,
            0x042c0100 => Self::CallEventByName6,
            0x03D20000 => Self::CallEventByVariable,
            0x03D30000 => Self::ReserveEvent,
            0x01000000 => Self::Exit,
            _ => Self::Unknown
        }
    }
}