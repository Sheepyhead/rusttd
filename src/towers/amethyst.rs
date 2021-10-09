use super::{
    cooldown_is_done, launch_projectile, AttackSpeed, Cooldown, Gem, GemQuality, GemType, Range,
    TowerBundle, BASE_TOWER_SPEED,
};
use crate::{
    abilities::{aura::Auras, OnHitAbilities},
    level_1::LevelState,
    towers::{Damage, Target},
};
use bevy::prelude::{self, *};

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::App) {
        app.add_system_set(SystemSet::on_update(LevelState::Spawning).with_system(attack));
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
        if !matches!(gem.r#type, GemType::Amethyst) {
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
            name: Name::new("Chipped amethyst"),
            damage: Damage::Range(10..=15),
            speed: AttackSpeed(BASE_TOWER_SPEED - 0.2),
            range: Range(10.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Flawed => TowerBundle {
            name: Name::new("Flawed amethyst"),
            damage: Damage::Range(20..=27),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(12.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Normal => TowerBundle {
            name: Name::new("Amethyst"),
            damage: Damage::Range(30..=45),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(13.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Flawless => TowerBundle {
            name: Name::new("Flawless amethyst"),
            damage: Damage::Range(60..=80),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(13.5),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Perfect => TowerBundle {
            name: Name::new("Perfect amethyst"),
            damage: Damage::Range(140..=170),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(16.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
    }
}
