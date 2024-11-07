use serde::Serialize;

#[derive(Serialize)]
pub enum Target {
    SavedPosition,   // 0xefd8ffff,
    Hero,            // 0xffffffff,
    Target(u32)
}

impl Target {
    pub fn new(target: u32) -> Self {
        match target {
            0xefd8ffff => Self::SavedPosition,
            0xffffffff => Self::Hero,
            _ => Self::Target(target)
        }
    }
}