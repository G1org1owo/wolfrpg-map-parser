use serde::Serialize;

#[derive(Serialize)]
pub enum U32OrString {
    U32(u32),
    String(String),
}