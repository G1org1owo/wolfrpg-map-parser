use serde::Serialize;

#[derive(Serialize)]
pub enum MoveRoute {
    NoMove      = 0x00,
    Custom      = 0x01,
    Random      = 0x02,
    TowardsHero = 0x03,
    Unknown
}

impl MoveRoute {
    pub const fn new(route: u8) -> Self {
        match route {
            0x00 => MoveRoute::NoMove,
            0x01 => MoveRoute::Custom,
            0x02 => MoveRoute::Random,
            0x03 => MoveRoute::TowardsHero,
            _ => MoveRoute::Unknown
        }
    }
}