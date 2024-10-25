use serde::Serialize;
use crate::command::picture_command::show::base::Base;
use crate::command::picture_command::options::Options;
use crate::command::picture_command::show::same_colors_delay::SameColorsDelay;

#[derive(Serialize)]
pub enum State {
    Base(Base),
    SameColorsDelay(SameColorsDelay),
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
}