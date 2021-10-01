use bevy::prelude::*;
use rand::Rng;

use crate::towers::{emerald, sapphire, Damage, Range};

#[derive(Clone, Copy)]
pub enum OnHit {
    MultiplyDamage { chance: u32, multiplier: u64 },
    Splash(SplashEffect, Range),
    SapphireSlow(u32),
    EmeraldPoison { dps: u32, slow: u32, duration: f32 },
}

#[derive(Clone, Copy)]
pub enum SplashEffect {
    Multiplier { multiplier: f32 },
}

impl OnHit {
    #[allow(clippy::cast_precision_loss)]
    pub fn apply(self, target: Entity, commands: &mut Commands, damage: &mut u64, position: Vec3) {
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
            OnHit::SapphireSlow(amount) => {
                commands
                    .entity(target)
                    .insert(sapphire::Slowed(amount, Timer::from_seconds(4.0, false)));
            }
            OnHit::EmeraldPoison {
                dps,
                slow,
                duration,
            } => {
                commands.entity(target).insert(emerald::Poison {
                    slow,
                    duration_timer: Timer::from_seconds(duration, false),
                    damage_timer: Timer::from_seconds(1.0 / (dps as f32), true),
                });
            }
        }
    }
}
