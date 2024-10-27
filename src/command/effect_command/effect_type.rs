use serde::Serialize;
use crate::command::effect_command::character_effect_type::CharacterEffectType;
use crate::command::effect_command::map_effect_type::MapEffectType;
use crate::command::effect_command::picture_effect_type::PictureEffectType;

#[derive(Serialize)]
pub enum EffectType {
    Picture(PictureEffectType),
    Character(CharacterEffectType),
    Map(MapEffectType),
    Unknown
}