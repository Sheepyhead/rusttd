use super::{
    cooldown_is_done, get_closest_creep_within_range, launch_projectile, AttackSpeed, Cooldown,
    Gem, GemQuality, GemType, Range, TowerBundle, BASE_TOWER_SPEED,
};
use crate::{creeps, level_1::LevelState, towers::Damage};
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
        if !matches!(gem.r#type, GemType::Topaz) {
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
            damage: Damage::Fixed(4),
            speed: AttackSpeed(BASE_TOWER_SPEED - 0.2),
            range: Range(5.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
        },
        GemQuality::Flawed => TowerBundle {
            damage: Damage::Fixed(8),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(5.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
        },
        GemQuality::Normal => TowerBundle {
            damage: Damage::Fixed(14),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(5.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
        },
        GemQuality::Flawless => TowerBundle {
            damage: Damage::Fixed(25),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(5.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
        },
        GemQuality::Perfect => TowerBundle {
            damage: Damage::Fixed(75),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(5.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
        },
    }
}
