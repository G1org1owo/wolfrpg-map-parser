mod models;
mod parsers;

pub(crate) const DATA_MAGIC: &[u8] = b"\x00\x57\x00\x00\x4F\x4C\x00\x46\x4D\x00";

pub use models::table;
pub use models::tileset;
pub use models::tile;
pub use models::type_info;
pub use models::game_data;

pub use parsers::data_parser;
pub use parsers::project_parser;
pub use parsers::tileset_parser;
pub use parsers::game_data_parser;