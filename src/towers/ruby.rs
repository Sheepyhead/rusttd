use super::{
    cooldown_is_done, launch_projectile, AttackSpeed, Cooldown, Gem, GemQuality, GemType, Range,
    TowerBundle, BASE_TOWER_SPEED,
};
use crate::{
    abilities::{
        aura::Auras,
        on_hit::{OnHit, SplashEffect},
        OnHitAbilities,
    },
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
        if !matches!(gem.r#type, GemType::Ruby) {
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
    use OnHit::*;
    use SplashEffect::*;
    match quality {
        GemQuality::Chipped => TowerBundle {
            name: Name::new("Chipped ruby"),
            damage: Damage::Range(8..=9),
            speed: AttackSpeed(BASE_TOWER_SPEED - 0.2),
            range: Range(8.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![Splash(Multiplier { multiplier: 0.5 }, Range(3.0))]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Flawed => TowerBundle {
            name: Name::new("Flawed ruby"),
            damage: Damage::Range(13..=16),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(8.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![Splash(Multiplier { multiplier: 0.5 }, Range(3.0))]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Normal => TowerBundle {
            name: Name::new("Ruby"),
            damage: Damage::Range(20..=25),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(8.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![Splash(Multiplier { multiplier: 0.5 }, Range(3.0))]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Flawless => TowerBundle {
            name: Name::new("Flawless ruby"),
            damage: Damage::Range(38..=45),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(8.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![Splash(Multiplier { multiplier: 0.5 }, Range(3.0))]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Perfect => TowerBundle {
            name: Name::new("Perfect ruby"),
            damage: Damage::Range(80..=100),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(8.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![Splash(Multiplier { multiplier: 0.5 }, Range(3.5))]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
    }
}
