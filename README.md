Parser for Wolf RPG Editor map files
====================================
[<img alt="github" src="https://img.shields.io/badge/github-G1org1owo/wolfrpg--map--parser-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/G1org1owo/wolfrpg-map-parser)
[<img alt="Crates.io Version" src="https://img.shields.io/crates/v/wolfrpg-map-parser?style=for-the-badge" height="20">](https://crates.io/crates/wolfrpg-map-parser)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-wolfprg--map--parser-66c2a5?style=for-the-badge" height="20">](https://docs.rs/wolfrpg-map-parser)

The aim of this crate is to allow users to easily parse Wolf RPG Editor map (`.mps`) files and expose a complete 
interface to enable interaction with each component of a map, from the tiles to the events.

This package includes both a library crate that parses the map into a tree of rust structs and a binary crate that
outputs the result in JSON format.