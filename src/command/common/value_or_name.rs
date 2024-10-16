use serde::Serialize;

#[derive(Serialize)]
pub enum ValueOrName {
    Value(u32),
    Name(String),
}