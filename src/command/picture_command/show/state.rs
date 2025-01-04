use serde::Serialize;
use crate::command::picture_command::show::base::Base;
use crate::command::picture_command::options::Options;
use crate::command::picture_command::show::color_values::ColorValues;
use crate::command::picture_command::show::colors::Colors;
use crate::command::picture_command::show::free_transform::FreeTransform;
use crate::command::picture_command::show::delay::Delay;
use crate::command::picture_command::show::range::Range;
use crate::command::picture_command::show::zoom::Zoom;

#[derive(Serialize)]
pub enum State {
    Base(Base),
    Colors(Colors),
    Delay(Delay),
    Range(Range),
    ColorValues(ColorValues),
    Zoom(Zoom),
    FreeTransform(FreeTransform)
}

impl State {
    pub fn parse_base(bytes: &[u8], options: &Options) -> (usize, Option<u32>, Self) {
        let (bytes_read, filename_variable, state): (usize, Option<u32>, Base)
            = Base::parse(bytes, options);

        (bytes_read, filename_variable, Self::Base(state))
    }

    pub fn parse_colors(bytes: &[u8], _: &Options) -> (usize, Option<u32>, Self) {
        let (bytes_read, filename_variable, state): (usize, Option<u32>, Colors)
            = Colors::parse(bytes);

        (bytes_read, filename_variable, Self::Colors(state))
    }

    pub fn parse_delay(bytes: &[u8], _: &Options) -> (usize, Option<u32>, State) {
        let (bytes_read, filename_variable, state): (usize, Option<u32>, Delay)
            = Delay::parse(bytes);

        (bytes_read, filename_variable, Self::Delay(state))
    }

    pub fn parse_range(bytes: &[u8], _: &Options) -> (usize, Option<u32>, State) {
        let (bytes_read, filename_variable, state): (usize, Option<u32>, Range)
            = Range::parse(bytes);

        (bytes_read, filename_variable, Self::Range(state))
    }

    pub fn parse_color_values(bytes: &[u8], _: &Options) -> (usize, Option<u32>, State) {
        let (bytes_read, filename_variable, state): (usize, Option<u32>, ColorValues)
            = ColorValues::parse(bytes);

        (bytes_read, filename_variable, Self::ColorValues(state))
    }

    pub fn parse_zoom(bytes: &[u8], _: &Options) -> (usize, Option<u32>, State) {
        let (bytes_read, filename_variable, state): (usize, Option<u32>, Zoom)
            = Zoom::parse(bytes);

        (bytes_read, filename_variable, Self::Zoom(state))
    }

    pub fn parse_free_transform(bytes: &[u8], _: &Options) -> (usize, Option<u32>, State) {
        let (bytes_read, filename_variable, state): (usize, Option<u32>, FreeTransform)
            = FreeTransform::parse(bytes);

        (bytes_read, filename_variable, Self::FreeTransform(state))
    }
}