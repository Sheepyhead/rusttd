use super::{
    cooldown_is_done, launch_projectile, AttackSpeed, Cooldown, Gem, GemQuality, GemType, Range,
    TowerBundle, BASE_TOWER_SPEED,
};
use crate::{
    abilities::{aura::Auras, on_hit::OnHit, OnHitAbilities},
    creeps::{self, Type},
    level_1::LevelState,
    towers::{Damage, Target},
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
        &Target,
        &mut Cooldown,
    )>,
    creeps: Query<(Entity, &GlobalTransform, &creeps::Type)>,
) {
    for (gem_entity, gem_position, gem, AttackSpeed(speed), Target(target), mut cooldown) in
        gems.iter_mut()
    {
        if !matches!(gem.r#type, GemType::Diamond) {
            continue;
        }

        if !cooldown_is_done(&mut *cooldown, *speed, &time) {
            continue;
        }

        if let Some(target) = target {
            if let Ok(r#type) = creeps.get_component::<Type>(*target) {
                match r#type {
                    Type::Ground => {}
                    Type::Flying => continue,
                }
            }

            cooldown.0.reset();

            launch_projectile(
                &mut commands,
                &mut meshes,
                gem_position,
                gem_entity,
                *target,
            );
        }
    }
}

pub fn tower(quality: GemQuality) -> TowerBundle {
    use OnHit::*;
    match quality {
        GemQuality::Chipped => TowerBundle {
            name: Name::new("Chipped diamond"),
            damage: Damage::Range(8..=12),
            speed: AttackSpeed(BASE_TOWER_SPEED - 0.2),
            range: Range(5.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![MultiplyDamage {
                chance: 25,
                multiplier: 2,
            }]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Flawed => TowerBundle {
            name: Name::new("Flawed diamond"),
            damage: Damage::Range(16..=18),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(5.5),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![MultiplyDamage {
                chance: 25,
                multiplier: 2,
            }]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Normal => TowerBundle {
            name: Name::new("Diamond"),
            damage: Damage::Range(30..=37),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(6.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![MultiplyDamage {
                chance: 25,
                multiplier: 2,
            }]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Flawless => TowerBundle {
            name: Name::new("Flawless diamond"),
            damage: Damage::Range(58..=65),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(6.5),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![MultiplyDamage {
                chance: 25,
                multiplier: 2,
            }]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Perfect => TowerBundle {
            name: Name::new("Perfect diamond"),
            damage: Damage::Range(140..=150),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(7.5),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![MultiplyDamage {
                chance: 25,
                multiplier: 2,
            }]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
    }
}
