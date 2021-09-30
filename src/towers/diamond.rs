use super::{
    cooldown_is_done, get_closest_creep_within_range, launch_projectile, AttackSpeed, Cooldown,
    Gem, GemQuality, GemType, Range, TowerBundle, BASE_TOWER_SPEED,
};
use crate::{
    abilities::{aura::Auras, on_hit::OnHit, OnHitAbilities},
    creeps::{self, Type},
    level_1::LevelState,
    towers::Damage,
};
use bevy::prelude::{self, *};

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::AppBuilder) {
        app.add_system_set(SystemSet::on_update(LevelState::Spawning).with_system(attack.system()));
    }
}

fn attack(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gems: Query<(
        Entity,
        &GlobalTransform,
        &Gem,
        &AttackSpeed,
        &Range,
        &mut Cooldown,
    )>,
    creeps: Query<(Entity, &GlobalTransform, &creeps::Type)>,
) {
    for (gem_entity, gem_position, gem, AttackSpeed(speed), Range(range), mut cooldown) in
        gems.iter_mut()
    {
        if !matches!(gem.r#type, GemType::Diamond) {
            continue;
        }

        if !cooldown_is_done(&mut *cooldown, *speed, &time) {
            continue;
        }

        if let Some(closest_creep) =
            get_closest_creep_within_range(&creeps, gem_position, *range, Some(Type::Ground))
        {
            launch_projectile(
                &mut commands,
                &mut meshes,
                gem_position,
                gem_entity,
                closest_creep,
            );
        }
    }
}

pub fn tower(quality: GemQuality) -> TowerBundle {
    use OnHit::*;
    match quality {
        GemQuality::Chipped => TowerBundle {
            damage: Damage::Range(8..=12),
            speed: AttackSpeed(BASE_TOWER_SPEED - 0.2),
            range: Range(5.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![MultiplyDamage {
                chance: 25,
                multiplier: 2,
            }]),
            auras: Auras(vec![]),
        },
        GemQuality::Flawed => TowerBundle {
            damage: Damage::Range(16..=18),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(5.5),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![MultiplyDamage {
                chance: 25,
                multiplier: 2,
            }]),
            auras: Auras(vec![]),
        },
        GemQuality::Normal => TowerBundle {
            damage: Damage::Range(30..=37),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(6.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![MultiplyDamage {
                chance: 25,
                multiplier: 2,
            }]),
            auras: Auras(vec![]),
        },
        GemQuality::Flawless => TowerBundle {
            damage: Damage::Range(58..=65),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(6.5),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![MultiplyDamage {
                chance: 25,
                multiplier: 2,
            }]),
            auras: Auras(vec![]),
        },
        GemQuality::Perfect => TowerBundle {
            damage: Damage::Range(140..=150),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(7.5),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![MultiplyDamage {
                chance: 25,
                multiplier: 2,
            }]),
            auras: Auras(vec![]),
        },
    }
}
