# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.2] - 2025-02-21

### Added

- Base version of `db_parser::game_data_parser`

### Changed

- Separated `db_parser` files in submodules `parsers` and `models`

### Fixed

- Small typos in documentation

## [0.4.1] - 2025-01-20

### Added

- `Clone` derive for all classes

## [0.4.0] - 2025-01-19

### Added

- New module to parse WolfRPG Editor databases and tilesets, `crate::db_parser`
- Documentation for `crate::db_parser`

### Changed

- `U32OrString` has been moved to `crate::common::u32_or_string::U32OrString`

### Fixed

- Case end not being detected properly on different nesting levels

## [0.3.3] - 2025-01-17

### Fixed

- Actual fix for Certain signatures for `CommonEventCommand` not being recognized

## [0.3.2 (YANKED)] - 2025-01-17

### Fixed

- Certain signatures for `CommonEventCommand` not being recognized
- `Page::icon_row` not being read correctly

## [0.3.1] - 2025-01-10

### Added

- `PartialEq` derive on all structs and enums

## [0.3.0] - 2025-01-10

### Added

- `serde::Deserialize` derive on all structs and enums

### Removed

- public `parse` associated functions from all structs except `map`, `event`, `page` and `command`

## [0.2.3] - 2025-01-09

### Added

- This changelog
- Documentation comments for `lib`, `map`, `event`, `page` and `command`

### Fixed
- `README.md` example relying on `args`

## [0.2.2] - 2025-01-09

### Added

- Re-exported `wolfrpg-map-parser::map::Map` as `wolfrpg-map-parser::Map`
- Usage section in `README.md`

### Changed
- Use new export in `main.rs`

## [0.2.1] - 2025-01-08

### Added

- `README.md` draft

### Fixed
- `extra` folder and other files being published on Cargo 

## [0.2.0] - 2025-01-07

### Added

- `LICENSE.md`
- All of the code