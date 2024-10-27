use serde::Serialize;
use crate::command::effect_command::character_effect_type::CharacterEffectType;
use crate::command::effect_command::effect_target::EffectTarget;
use crate::command::effect_command::effect_type::EffectType;
use crate::command::effect_command::map_effect_type::MapEffectType;
use crate::command::effect_command::picture_effect_type::PictureEffectType;

#[derive(Serialize)]
pub struct BaseOptions {
    target: EffectTarget,
    effect_type: EffectType
}

impl BaseOptions {
    pub fn new(options: u32) -> Self {
        let target: EffectTarget = EffectTarget::new((options & 0x0f) as u8);
        let effect_type: u8 = ((options & 0xf0) >> 4) as u8;

        let effect_type: EffectType = match target {
            EffectTarget::Picture => EffectType::Picture(PictureEffectType::new(effect_type)),
            EffectTarget::Character => EffectType::Character(CharacterEffectType::new(effect_type)),
            EffectTarget::Map => EffectType::Map(MapEffectType::new(effect_type)),
            EffectTarget::Unknown => EffectType::Unknown
        };

        Self {
            target,
            effect_type
        }
    }
}