use bevy::prelude::{self, *};

use crate::{
    level_1::{map, LevelState},
    towers::{Gem, ProjectileHit},
};

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::AppBuilder) {
        app.add_event::<Death>()
            .add_system_set(
                SystemSet::on_enter(LevelState::Spawning).with_system(start_spawn.system()),
            )
            .add_system_set(
                SystemSet::on_update(LevelState::Spawning)
                    .with_system(spawn.system())
                    .with_system(moving.system())
                    .with_system(death.system())
                    .with_system(projectile_hit.system()),
            );
    }
}

struct Spawner {
    amount: u32,
    timer: Timer,
}

fn start_spawn(mut commands: Commands) {
    info!("Creating spawner");
    commands.spawn_bundle((
        Spawner {
            amount: 20,
            timer: Timer::from_seconds(1.0, true),
        },
        Transform::from_translation(Vec3::new(-20.0, 0.0, 2.0)),
        GlobalTransform::default(),
    ));
}

pub struct Creep {
    life: u64,
    route: Vec<(i32, i32)>,
    destination: usize,
}

fn spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut spawners: Query<(Entity, &Transform, &mut Spawner)>,
) {
    for (spawner_entity, transform, mut spawner) in spawners.iter_mut() {
        spawner.timer.tick(time.delta());

        if spawner.timer.just_finished() {
            info!(
                "Spawning creep #{} from {:?}",
                spawner.amount, spawner_entity
            );
            commands
                .spawn_bundle((Creep {
                    life: 20,
                    route: map::CREEP_ROUTE.to_vec(),
                    destination: 0,
                },))
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(
                        shape::Icosphere {
                            radius: 0.5,
                            subdivisions: 10,
                        }
                        .into(),
                    ),
                    transform: *transform,
                    ..PbrBundle::default()
                });

            spawner.amount -= 1;

            if spawner.amount == 0 {
                info!("Despawning spawner {:?}", spawner_entity);
                commands.entity(spawner_entity).despawn_recursive();
            }
        }
    }
}

#[allow(clippy::cast_precision_loss)]
fn moving(
    mut commands: Commands,
    time: Res<Time>,
    mut ew: EventWriter<Death>,
    mut creeps: Query<(Entity, &mut Transform, &mut Creep)>,
) {
    for (creep_entity, mut transform, mut creep) in creeps.iter_mut() {
        if let Some(destination) = creep.route.get(creep.destination) {
            let mut direction = Vec3::new(
                destination.0 as f32,
                transform.translation.y,
                destination.1 as f32,
            ) - transform.translation;
            direction = direction.normalize();
            direction *= 5.0 * time.delta_seconds();
            transform.translation += direction;

            if (transform.translation.x - destination.0 as f32).abs() <= 0.05
                && (transform.translation.z - destination.1 as f32).abs() <= 0.05
            {
                creep.destination += 1;
            }
        } else {
            // The creep has reached the end of its destination
            info!(
                "Creep {:?} has been leaked with {} life remaining",
                creep_entity, creep.life
            );
            ew.send(Death {
                _remaining_life: Some(creep.life),
            });
            commands.entity(creep_entity).despawn_recursive();
        }
    }
}

struct Death {
    _remaining_life: Option<u64>,
}

fn death(
    mut level_state: ResMut<State<LevelState>>,
    mut er: EventReader<Death>,
    creeps: Query<(), With<Creep>>,
) {
    for _ in er.iter() {
        if creeps.iter().count() == 0 {
            level_state
                .set(LevelState::Building)
                .map_err(|err| error!("Failed to set level state to building: {}", err))
                .ok();
        }
    }
}

fn projectile_hit(
    mut commands: Commands,
    mut er: EventReader<ProjectileHit>,
    mut ew: EventWriter<Death>,
    mut creeps: Query<&mut Creep>,
) {
    for ProjectileHit(projectile) in er.iter() {
        if let Ok(mut creep) = creeps.get_mut(projectile.target) {
            let damage = 20;
            if creep.life >= damage {
                creep.life -= damage;
            }

            if creep.life == 0 {
                ew.send(Death {
                    _remaining_life: None,
                });
                commands.entity(projectile.target).despawn_recursive();
            }
        }
    }
}
