use bevy::prelude::*;
use rand::Rng;

use crate::towers::{Damage, Range};

#[derive(Clone, Copy)]
pub enum OnHit {
    MultiplyDamage { chance: u32, multiplier: u64 },
    Splash(SplashEffect, Range),
}

#[derive(Clone, Copy)]
pub enum SplashEffect {
    Multiplier { multiplier: f32 },
}

impl OnHit {
    pub fn apply(self, commands: &mut Commands, damage: &mut u64, position: Vec3) {
        match self {
            OnHit::MultiplyDamage { chance, multiplier } => {
                if rand::thread_rng().gen_range(0..100) <= chance {
                    *damage *= multiplier;
                }
            }
            OnHit::Splash(effect, range) => {
                commands.spawn_bundle((
                    effect,
                    range,
                    Damage::Fixed(*damage),
                    Transform::from_translation(position),
                    GlobalTransform::default(),
                ));
            }
        }
    }
}
