pub mod project_parser;
pub mod data_parser;
pub mod tileset_parser;
pub mod type_info;
pub mod table;
pub mod tileset;
pub mod tile;

pub(crate) const DATA_MAGIC: &[u8] = b"\x00\x57\x00\x00\x4F\x4C\x00\x46\x4D\x00";