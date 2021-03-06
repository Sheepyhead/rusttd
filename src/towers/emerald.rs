use super::{
    cooldown_is_done, launch_projectile, AttackSpeed, Cooldown, Gem, GemQuality, GemType, Range,
    TowerBundle, BASE_TOWER_SPEED,
};
use crate::{
    abilities::{aura::Auras, on_hit::OnHit, OnHitAbilities},
    creeps::{damage_creep, Death, Life, Speed},
    level_1::LevelState,
    towers::{Damage, Target},
};
use bevy::prelude::{self, *};

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::App) {
        app.add_system_set(
            SystemSet::on_update(LevelState::Spawning)
                .with_system(attack)
                .with_system(Poison::added)
                .with_system(Poison::system),
        );
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
) {
    for (gem_entity, gem_position, gem, AttackSpeed(speed), Target(target), mut cooldown) in
        gems.iter_mut()
    {
        if !matches!(gem.r#type, GemType::Emerald) {
            continue;
        }

        if !cooldown_is_done(&mut *cooldown, *speed, &time) {
            continue;
        }

        if let Some(target) = target {
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
    match quality {
        GemQuality::Chipped => TowerBundle {
            name: Name::new("Chipped emerald"),
            damage: Damage::Range(4..=7),
            speed: AttackSpeed(BASE_TOWER_SPEED - 0.2),
            range: Range(5.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![OnHit::EmeraldPoison {
                dps: 2,
                slow: 15,
                duration: 3.0,
            }]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Flawed => TowerBundle {
            name: Name::new("Flawed emerald"),
            damage: Damage::Range(10..=13),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(5.5),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![OnHit::EmeraldPoison {
                dps: 3,
                slow: 20,
                duration: 4.0,
            }]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Normal => TowerBundle {
            name: Name::new("Emerald"),
            damage: Damage::Range(15..=25),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(6.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![OnHit::EmeraldPoison {
                dps: 5,
                slow: 25,
                duration: 5.0,
            }]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Flawless => TowerBundle {
            name: Name::new("Flawless emerald"),
            damage: Damage::Range(30..=37),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(7.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![OnHit::EmeraldPoison {
                dps: 8,
                slow: 30,
                duration: 6.0,
            }]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Perfect => TowerBundle {
            name: Name::new("Perfect emerald"),
            damage: Damage::Range(80..=95),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(7.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![OnHit::EmeraldPoison {
                dps: 16,
                slow: 50,
                duration: 8.0,
            }]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
    }
}

#[derive(Component)]
pub struct Poison {
    pub slow: u32,
    pub duration_timer: Timer,
    pub damage_timer: Timer,
}

impl Poison {
    pub fn added(mut poisoned_creeps: Query<(&Poison, &mut Speed), Added<Poison>>) {
        for (poison, mut speed) in poisoned_creeps.iter_mut() {
            speed.reduce(poison.slow);
        }
    }
    pub fn system(
        mut commands: Commands,
        time: Res<Time>,
        mut ew: EventWriter<Death>,
        mut poisoned_creeps: Query<(Entity, &mut Poison, &mut Speed, &mut Life)>,
    ) {
        for (entity, mut poison, mut speed, mut life) in poisoned_creeps.iter_mut() {
            if poison.duration_timer.tick(time.delta()).just_finished() {
                commands.entity(entity).remove::<Poison>();
                speed.increase(poison.slow);
            } else if poison.damage_timer.tick(time.delta()).just_finished() {
                damage_creep(entity, 1, &mut life, &mut ew);
            }
        }
    }
}
