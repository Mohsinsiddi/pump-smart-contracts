use anchor_lang::prelude::*;

use crate::constants::MAX_WOOD_PER_TREE;

#[account]
pub struct GameData {
    pub init_chances: u8,
}

impl GameData {
    pub fn on_token_bought(&mut self, chances: u8) -> Result<()> {
        match self.init_chances.checked_add(chances) {
            Some(v) => {
                if self.init_chances >= MAX_WOOD_PER_TREE {
                    self.init_chances = 0;
                    msg!("Tree successfully chopped. New Tree coming up.");
                } else {
                    self.init_chances = v;
                    msg!("Total wood chopped: {}", v);
                }
            }
            None => {
                msg!("The ever tree is completly chopped!");
            }
        };

        Ok(())
    }
}
