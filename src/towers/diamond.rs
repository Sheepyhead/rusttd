use super::{Gem, Projectile};
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
    mut gems: Query<(Entity, &GlobalTransform, &mut Gem)>,
    creeps: Query<(Entity, &GlobalTransform), With<Creep>>,
) {
    for (gem_entity, gem_position, mut gem) in gems.iter_mut() {
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
        if closest_distance >= 15.0 {
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
