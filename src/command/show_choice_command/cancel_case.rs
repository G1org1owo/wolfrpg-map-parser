use serde::Serialize;

#[derive(Serialize)]
pub enum CancelCase {
    Choice(u8),
    Separate,
    No
}

impl CancelCase {
    pub fn new(cancel_case: u8) -> Self {
        match cancel_case {
            0 => CancelCase::Separate,
            1 => CancelCase::No,
            _ => CancelCase::Choice(cancel_case - 2)
        }
    }
}