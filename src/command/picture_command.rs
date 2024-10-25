use serde::Serialize;
use crate::command::picture_command::show::Show;

mod show;
mod options;
mod display_type;
mod blending_method;
mod anchor;
mod zoom;
mod colors;

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

    pub fn parse_show_delay(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Show) = Show::parse_delay(bytes);

        (bytes_read, Self::Show(command))
    }

    pub fn parse_show_range(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Show) = Show::parse_range(bytes);

        (bytes_read, Self::Show(command))
    }

    pub fn parse_color_values(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Show) = Show::parse_color_values(bytes);

        (bytes_read, Self::Show(command))
    }

    pub fn parse_show_zoom(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Show) = Show::parse_zoom(bytes);

        (bytes_read, Self::Show(command))
    }

    pub fn parse_show_free_transform(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Show) = Show::parse_free_transform(bytes);

        (bytes_read, Self::Show(command))
    }
}