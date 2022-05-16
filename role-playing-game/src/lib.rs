// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }
        return Some(Player {
            health: 100,
            mana: if self.level > 9 { Some(100) } else { None },
            ..*self
        });
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana == None {
            self.health = self.health.saturating_sub(mana_cost);
        } else {
            if let Some(mana) = self.mana {
                if mana >= mana_cost {
                    self.mana = Some(mana - mana_cost);
                    return mana_cost * 2;
                }
            }
        }
        return 0;
    }
}
