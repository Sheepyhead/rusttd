use super::{
    cooldown_is_done, get_closest_creep_within_range, launch_projectile, AttackSpeed, Cooldown,
    Gem, GemQuality, GemType, Range, TowerBundle, BASE_TOWER_SPEED,
};
use crate::{
    abilities::{
        aura::{Aura, Auras, Kind},
        OnHitAbilities,
    },
    creeps,
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
        if !matches!(gem.r#type, GemType::Opal) {
            continue;
        }

        if !cooldown_is_done(&mut *cooldown, *speed, &time) {
            continue;
        }

        if let Some(closest_creep) =
            get_closest_creep_within_range(&creeps, gem_position, *range, None)
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
    match quality {
        GemQuality::Chipped => TowerBundle {
            damage: Damage::Fixed(5),
            speed: AttackSpeed(BASE_TOWER_SPEED - 0.2),
            range: Range(6.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![]),
            auras: Auras(vec![Aura {
                range: Range(6.0),
                kind: Kind::AttackSpeed(0.1),
            }]),
        },
        GemQuality::Flawed => TowerBundle {
            damage: Damage::Fixed(10),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(7.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![]),
            auras: Auras(vec![Aura {
                range: Range(7.0),
                kind: Kind::AttackSpeed(0.15),
            }]),
        },
        GemQuality::Normal => TowerBundle {
            damage: Damage::Fixed(20),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(8.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![]),
            auras: Auras(vec![Aura {
                range: Range(8.0),
                kind: Kind::AttackSpeed(0.2),
            }]),
        },
        GemQuality::Flawless => TowerBundle {
            damage: Damage::Fixed(40),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(9.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![]),
            auras: Auras(vec![Aura {
                range: Range(9.0),
                kind: Kind::AttackSpeed(0.25),
            }]),
        },
        GemQuality::Perfect => TowerBundle {
            damage: Damage::Fixed(85),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(10.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![]),
            auras: Auras(vec![Aura {
                range: Range(10.0),
                kind: Kind::AttackSpeed(0.35),
            }]),
        },
    }
}
