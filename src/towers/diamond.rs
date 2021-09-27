use std::time::Duration;

use super::{AttackSpeed, Gem, Projectile, Range};
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
    mut gems: Query<(Entity, &GlobalTransform, &mut Gem, &AttackSpeed, &Range)>,
    creeps: Query<(Entity, &GlobalTransform), With<Creep>>,
) {
    for (gem_entity, gem_position, mut gem, AttackSpeed(speed), Range(range)) in gems.iter_mut() {
        gem.cooldown
            .set_duration(Duration::from_secs_f32(1.0 * speed));
        gem.cooldown.tick(time.delta());
        if !gem.cooldown.just_finished() {
            continue;
        }

        let mut closest = None;
        let mut closest_distance = f32::INFINITY;
        for (creep, position) in creeps.iter() {
            let distance = gem_position
                .translation
                .distance_squared(position.translation);

            if distance < closest_distance {
                closest = Some(creep);
                closest_distance = distance;
            }
        }
        if closest_distance >= range * 2.0 {
            continue;
        }
        if let Some(closest_creep) = closest {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(
                        shape::Icosphere {
                            radius: 0.1,
                            subdivisions: 5,
                        }
                        .into(),
                    ),
                    transform: Transform::from_translation(gem_position.translation),
                    ..PbrBundle::default()
                })
                .insert(Projectile {
                    origin: gem_entity,
                    target: closest_creep,
                });
        }
    }
}
