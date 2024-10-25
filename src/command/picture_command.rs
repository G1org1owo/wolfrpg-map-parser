use serde::Serialize;
use crate::command::picture_command::show::Show;

mod show;
mod options;
mod display_type;
mod blending_method;
mod anchor;
mod zoom;

#[derive(Serialize)]
pub enum PictureCommand {
    Show(Show),
    Erase
}

impl PictureCommand {
    pub fn parse_show_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Show) = Show::parse_base(bytes);

        (bytes_read, Self::Show(command))
    }

    pub fn parse_show_same_colors_delay(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Show) = Show::parse_same_colors_delay(bytes);

        (bytes_read, Self::Show(command))
    }
}