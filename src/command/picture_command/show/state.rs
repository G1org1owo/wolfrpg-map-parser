use serde::Serialize;
use crate::command::picture_command::show::base::Base;
use crate::command::picture_command::options::Options;
use crate::command::picture_command::show::free_transform::FreeTransform;
use crate::command::picture_command::show::same_colors_delay::SameColorsDelay;
use crate::command::picture_command::show::zoom::Zoom;

#[derive(Serialize)]
pub enum State {
    Base(Base),
    SameColorsDelay(SameColorsDelay),
    Zoom(Zoom),
    FreeTransform(FreeTransform)
}

impl State {
    pub fn parse_base(bytes: &[u8], options: &Options) -> (usize, Self) {
        let (bytes_read, state): (usize, Base) = Base::parse(bytes, options);

        (bytes_read, Self::Base(state))
    }

    pub fn parse_same_colors_delay(bytes: &[u8], options: &Options) -> (usize, State) {
        let (bytes_read, state): (usize, SameColorsDelay) = SameColorsDelay::parse(bytes, options);

        (bytes_read, Self::SameColorsDelay(state))
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