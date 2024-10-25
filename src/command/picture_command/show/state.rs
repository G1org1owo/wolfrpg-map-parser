use serde::Serialize;
use crate::command::picture_command::show::base::Base;
use crate::command::picture_command::options::Options;
use crate::command::picture_command::show::color_values::ColorValues;
use crate::command::picture_command::show::free_transform::FreeTransform;
use crate::command::picture_command::show::delay::Delay;
use crate::command::picture_command::show::zoom::Zoom;

#[derive(Serialize)]
pub enum State {
    Base(Base),
    Delay(Delay),
    ColorValues(ColorValues),
    Zoom(Zoom),
    FreeTransform(FreeTransform)
}

impl State {
    pub fn parse_base(bytes: &[u8], options: &Options) -> (usize, Self) {
        let (bytes_read, state): (usize, Base) = Base::parse(bytes, options);

        (bytes_read, Self::Base(state))
    }

    pub fn parse_delay(bytes: &[u8], options: &Options) -> (usize, State) {
        let (bytes_read, state): (usize, Delay) = Delay::parse(bytes, options);

        (bytes_read, Self::Delay(state))
    }

    pub fn parse_color_values(bytes: &[u8], options: &Options) -> (usize, State) {
        let (bytes_read, state): (usize, ColorValues) = ColorValues::parse(bytes, options);

        (bytes_read, Self::ColorValues(state))
    }

    pub fn parse_zoom(bytes: &[u8], options: &Options) -> (usize, State) {
        let (bytes_read, state): (usize, _) = Zoom::parse(bytes, options);

        (bytes_read, Self::Zoom(state))
    }

    pub fn parse_free_transform(bytes: &[u8], options: &Options) -> (usize, State) {
        let (bytes_read, state): (usize, FreeTransform) = FreeTransform::parse(bytes, options);

        (bytes_read, Self::FreeTransform(state))
    }
}