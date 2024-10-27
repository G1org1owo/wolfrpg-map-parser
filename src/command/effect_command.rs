use serde::Serialize;
use crate::command::effect_command::base::Base;

mod base;
mod base_options;
mod effect_target;
mod effect_type;
mod picture_effect_type;
mod character_effect_type;
mod map_effect_type;

#[derive(Serialize)]
pub enum EffectCommand {
    Base(Base),
    MapShake,
    ScrollScreen,
    ChangeColor
}

impl EffectCommand {
    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Base) = Base::parse(bytes);

        (bytes_read, Self::Base(command))
    }
}