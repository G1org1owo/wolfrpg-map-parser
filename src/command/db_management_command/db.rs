use serde::Serialize;

#[derive(Serialize)]
pub enum DB {
    VarDB  = 0b00000000,
    SysDB  = 0b00000001,
    UserDB = 0b00000010,
    Unknown
}

impl DB {
    pub const fn new(db: u8) -> Self {
        match db {
            0b00000000 => DB::VarDB,
            0b00000001 => DB::SysDB,
            0b00000010 => DB::UserDB,
            _ => DB::Unknown
        }
    }
}