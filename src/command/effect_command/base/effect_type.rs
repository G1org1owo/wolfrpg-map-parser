use serde::Serialize;
use crate::command::effect_command::base::character_effect_type::CharacterEffectType;
use crate::command::effect_command::base::map_effect_type::MapEffectType;
use crate::command::effect_command::base::picture_effect_type::PictureEffectType;

#[derive(Serialize)]
pub enum EffectType {
    Picture(PictureEffectType),
    Character(CharacterEffectType),
    Map(MapEffectType),
    Unknown
}