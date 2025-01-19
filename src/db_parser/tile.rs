#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq)]
pub struct Tile {
    tag_number: u8,
    down_not_passable: bool,
    left_not_passable: bool,
    right_not_passable: bool,
    up_not_passable: bool,
    always_above_character: bool,
    bottom_half_translucent: bool,
    conceal_character_behind: bool,
    match_passable_under: bool
}

impl Tile {
    pub fn new(tag_number: u8, options: u32) -> Self {
        Self {
            tag_number,
            down_not_passable: options & 0b00000001 != 0,
            left_not_passable: options & 0b00000010 != 0,
            right_not_passable: options & 0b00000100 != 0,
            up_not_passable: options & 0b00001000 != 0,
            always_above_character: options & 0b00010000 != 0,
            
            bottom_half_translucent: options & 0b01000000 != 0,
            
            conceal_character_behind: (options >> 8) & 0b00000001 != 0,
            match_passable_under: (options >> 8) & 0b00000010 != 0
        }
    }

    pub fn tag_number(&self) -> u8 {
        self.tag_number
    }
    
    pub fn tag_number_mut(&mut self) -> &mut u8 {
        &mut self.tag_number
    }

    pub fn down_not_passable(&self) -> bool {
        self.down_not_passable
    }
    
    pub fn down_not_passable_mut(&mut self) -> &mut bool {
        &mut self.down_not_passable
    }

    pub fn left_not_passable(&self) -> bool {
        self.left_not_passable
    }
    
    pub fn left_not_passable_mut(&mut self) -> &mut bool {
        &mut self.left_not_passable
    }

    pub fn right_not_passable(&self) -> bool {
        self.right_not_passable
    }
    
    pub fn right_not_passable_mut(&mut self) -> &mut bool {
        &mut self.right_not_passable
    }

    pub fn up_not_passable(&self) -> bool {
        self.up_not_passable
    }
    
    pub fn up_not_passable_mut(&mut self) -> &mut bool {
        &mut self.up_not_passable
    }

    pub fn always_above_character(&self) -> bool {
        self.always_above_character
    }
    
    pub fn always_above_character_mut(&mut self) -> &mut bool {
        &mut self.always_above_character
    }

    pub fn bottom_half_translucent(&self) -> bool {
        self.bottom_half_translucent
    }
    
    pub fn bottom_half_translucent_mut(&mut self) -> &mut bool {
        &mut self.bottom_half_translucent
    }

    pub fn conceal_character_behind(&self) -> bool {
        self.conceal_character_behind
    }
    
    pub fn conceal_character_behind_mut(&mut self) -> &mut bool {
        &mut self.conceal_character_behind
    }

    pub fn match_passable_under(&self) -> bool {
        self.match_passable_under
    }
    
    pub fn match_passable_under_mut(&mut self) -> &mut bool {
        &mut self.match_passable_under
    }
}