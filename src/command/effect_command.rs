use serde::Serialize;
use crate::command::effect_command::base::Base;
use crate::command::effect_command::map_shake::MapShake;

mod base;
mod map_shake;

#[derive(Serialize)]
pub enum EffectCommand {
    Base(Base),
    MapShake(MapShake),
    ScrollScreen,
    ChangeColor
}

impl EffectCommand {
    pub fn parse_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Base) = Base::parse(bytes);

        (bytes_read, Self::Base(command))
    }

    pub fn parse_map_shake(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, MapShake) = MapShake::parse(bytes);

        (bytes_read, Self::MapShake(command))
    }
}