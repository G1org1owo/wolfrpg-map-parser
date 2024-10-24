use serde::Serialize;
use crate::command::picture_command::show::base::Base;
use crate::command::picture_command::options::Options;

#[derive(Serialize)]
pub enum State {
    Base(Base),

}

impl State {
    pub fn parse_base(bytes: &[u8], options: &Options) -> (usize, Self) {
        let (bytes_read, state): (usize, Base) = Base::parse(bytes, options);

        (bytes_read, Self::Base(state))
    }
}