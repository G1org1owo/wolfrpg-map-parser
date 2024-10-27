use serde::Serialize;
use crate::command::sound_command::filename::Filename;
use crate::command::sound_command::options::Options;
use crate::command::sound_command::variable::Variable;

#[derive(Serialize)]
pub enum State {
    Filename(Filename),
    Variable(Variable),
    FreeAll
}

impl State {
    pub fn parse_filename(bytes: &[u8], options: &Options) -> (usize, Self) {
        let (bytes_read, state): (usize, Filename) = Filename::parse(bytes, options);

        (bytes_read, Self::Filename(state))
    }

    pub fn parse_variable(bytes: &[u8], options: &Options) -> (usize, Self) {
        let (bytes_read, state): (usize, Variable) = Variable::parse(bytes, options);

        (bytes_read, Self::Variable(state))
    }
}