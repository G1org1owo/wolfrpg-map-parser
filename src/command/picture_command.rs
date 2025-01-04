use serde::Serialize;
use crate::command::picture_command::erase::Erase;
use crate::command::picture_command::show::Show;

pub mod show;
pub mod options;
pub mod display_type;
pub mod blending_method;
pub mod anchor;
pub mod zoom;
pub mod colors;
mod erase;

#[derive(Serialize)]
pub enum PictureCommand {
    Show(Show),
    Erase(Erase),
}

impl PictureCommand {
    pub fn parse_show_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Show) = Show::parse_base(bytes);

        (bytes_read, Self::Show(command))
    }

    pub fn parse_show_colors(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Show) = Show::parse_colors(bytes);

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

    pub fn parse_erase_delay_reset(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Erase) = Erase::parse_delay_reset(bytes);

        (bytes_read, Self::Erase(command))
    }

    pub fn parse_erase_base(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Erase) = Erase::parse_base(bytes);

        (bytes_read, Self::Erase(command))
    }

    pub fn parse_erase_delay(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Erase) = Erase::parse_delay(bytes);

        (bytes_read, Self::Erase(command))
    }

    pub fn parse_erase_range(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, Erase) = Erase::parse_range(bytes);

        (bytes_read, Self::Erase(command))
    }
}