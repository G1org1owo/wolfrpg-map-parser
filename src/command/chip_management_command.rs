use serde::Serialize;
use crate::command::chip_management_command::map_chip_settings::MapChipSettings;

mod map_chip_settings;
mod options;

#[derive(Serialize)]
pub enum ChipManagementCommand {
    MapChipSettings(MapChipSettings),
    SwitchChipset,
    OverwriteMapChips
}

impl ChipManagementCommand {
    pub fn parse_map_chip_settings(bytes: &[u8]) -> (usize, Self) {
        let (bytes_read, command): (usize, MapChipSettings) = MapChipSettings::parse(bytes);

        (bytes_read, Self::MapChipSettings(command))
    }
}