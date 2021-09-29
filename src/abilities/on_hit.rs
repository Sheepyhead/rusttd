use bevy::prelude::*;
use rand::Rng;

pub enum OnHit {
    MultiplyDamage { chance: u32, multiplier: u64 },
}

impl OnHit {
    pub fn apply(&self, damage: &mut u64) {
        match self {
            OnHit::MultiplyDamage { chance, multiplier } => {
                info!("multiply damage happened!");
                if rand::thread_rng().gen_range(0..100) <= *chance {
                    info!("hit!");
                    *damage *= multiplier;
                }
            }
        }
    }
}
