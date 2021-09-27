use super::{
    ccoldown_is_done, get_closest_creep_within_range, launch_projectile, AttackSpeed, Cooldown,
    Gem, GemType, Range,
};
use crate::{creeps::Creep, level_1::LevelState};
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
    creeps: Query<(Entity, &GlobalTransform), With<Creep>>,
) {
    for (gem_entity, gem_position, gem, AttackSpeed(speed), Range(range), mut cooldown) in
        gems.iter_mut()
    {
        if !matches!(gem.r#type, GemType::Diamond) {
            continue;
        }

        if !ccoldown_is_done(&mut *cooldown, *speed, &time) {
            continue;
        }

        if let Some(closest_creep) = get_closest_creep_within_range(&creeps, gem_position, *range) {
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
