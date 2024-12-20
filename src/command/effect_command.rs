use serde::Serialize;
use crate::command::effect_command::base::Base;
use crate::command::effect_command::change_color::ChangeColor;
use crate::command::effect_command::map_shake::MapShake;
use crate::command::effect_command::scroll_screen::ScrollScreen;

mod base;
mod map_shake;
mod scroll_screen;
mod change_color;

#[derive(Serialize)]
pub enum EffectCommand {
    Base(Base),
    MapShake(MapShake),
    ScrollScreen(ScrollScreen),
    ChangeColor(ChangeColor)
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

    pub fn parse_scroll_screen(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, ScrollScreen) = ScrollScreen::parse(bytes);

        (bytes_read, Self::ScrollScreen(command))
    }

    pub fn parse_change_color(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, ChangeColor) = ChangeColor::parse(bytes);

        (bytes_read, Self::ChangeColor(command))
    }
}