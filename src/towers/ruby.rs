use super::{
    cooldown_is_done, get_closest_creep_within_range, launch_projectile, AttackSpeed, Cooldown,
    Gem, GemQuality, GemType, Range, TowerBundle, BASE_TOWER_SPEED,
};
use crate::{
    abilities::{
        aura::Auras,
        on_hit::{OnHit, SplashEffect},
        OnHitAbilities,
    },
    creeps,
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
        &Range,
        &mut Cooldown,
    )>,
    creeps: Query<(Entity, &GlobalTransform, &creeps::Type)>,
) {
    for (gem_entity, gem_position, gem, AttackSpeed(speed), Range(range), mut cooldown) in
        gems.iter_mut()
    {
        if !matches!(gem.r#type, GemType::Ruby) {
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
    use OnHit::*;
    use SplashEffect::*;
    match quality {
        GemQuality::Chipped => TowerBundle {
            damage: Damage::Range(8..=9),
            speed: AttackSpeed(BASE_TOWER_SPEED - 0.2),
            range: Range(8.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![Splash(Multiplier { multiplier: 0.5 }, Range(3.0))]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Flawed => TowerBundle {
            damage: Damage::Range(13..=16),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(8.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![Splash(Multiplier { multiplier: 0.5 }, Range(3.0))]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Normal => TowerBundle {
            damage: Damage::Range(20..=25),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(8.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![Splash(Multiplier { multiplier: 0.5 }, Range(3.0))]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Flawless => TowerBundle {
            damage: Damage::Range(38..=45),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(8.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![Splash(Multiplier { multiplier: 0.5 }, Range(3.0))]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Perfect => TowerBundle {
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
