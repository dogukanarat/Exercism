// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

#[derive(Debug, Clone)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let mut health = self.health;
        let mut mana = self.mana;
        let mut level = self.level;

        if(self.health == 0) {
            health = 100;
        }

        if(self.level >= 10) {
            mana = Some(100);
        } else {
            mana = None;
        }

        if(health <= 1) {
            None
        } else {
            Some(Player {
                health: health,
                mana: mana,
                level: level,
            })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if (mana >= mana_cost) {
                let new_mana = mana - mana_cost;
                self.mana = Some(new_mana);
                mana_cost * 2
            } else {
                0
            }
        } else {
            if (self.health < mana_cost) {
                self.health = 0;
            } else {
                self.health = self.health - mana_cost;
            }
            0
        }
    }
}
