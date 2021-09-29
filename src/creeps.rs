use bevy::prelude::{self, *};
use rand::Rng;

use crate::{
    grid::Grid,
    level_1::{map, LevelState},
    math_utils,
    path::resolve,
    towers::{Damage, ProjectileHit},
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
            amount: 10,
            timer: Timer::from_seconds(1.0, true),
        },
        Transform::from_translation(Vec3::new(-20.0, 0.0, 2.0)),
        GlobalTransform::default(),
    ));
}

#[derive(Bundle)]
pub struct CreepBundle {
    life: Life,
    movement: Movement,
    r#type: Type,
}

pub struct Life(u64);

pub struct Movement {
    route: Vec<(i32, i32)>,
    destination: usize,
}

fn spawn(
    mut commands: Commands,
    time: Res<Time>,
    grid: Res<Grid>,
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
            let mut route_for_spawner = vec![Grid::to_grid_pos(transform.translation / 2.0)];
            route_for_spawner.extend(map::CREEP_ROUTE.iter());
            let route =
                resolve(&*grid, &route_for_spawner).unwrap_or_else(|| map::CREEP_ROUTE.to_vec());
            commands
                .spawn_bundle((CreepBundle {
                    life: Life(20),
                    movement: Movement {
                        route: route.clone(),
                        destination: 0,
                    },
                    r#type: Type::Flying,
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
    time: Res<Time>,
    mut ew: EventWriter<Death>,
    mut creeps: Query<(Entity, &mut Transform, &mut Movement, &Life)>,
) {
    for (creep_entity, mut transform, mut movement, Life(life)) in creeps.iter_mut() {
        if let Some(destination) = movement.route.get(movement.destination) {
            let speed = 5.0 * time.delta_seconds();

            transform.translation = math_utils::move_towards(
                transform.translation,
                Vec3::new(
                    destination.0 as f32,
                    transform.translation.y,
                    destination.1 as f32,
                ),
                speed,
            );
            if (transform.translation.x - destination.0 as f32).abs() <= 0.05
                && (transform.translation.z - destination.1 as f32).abs() <= 0.05
            {
                movement.destination += 1;
            }
        } else {
            // The creep has reached the end of its destination
            info!(
                "Creep {:?} has been leaked with {} life remaining",
                creep_entity, life
            );
            ew.send(Death {
                remaining_life: Some(*life),
                entity: creep_entity,
            });
        }
    }
}

struct Death {
    remaining_life: Option<u64>,
    entity: Entity,
}

fn death(
    mut commands: Commands,
    mut level_state: ResMut<State<LevelState>>,
    mut er: EventReader<Death>,
    creeps: Query<(), With<Type>>,
) {
    let mut deaths = 0;
    for Death {
        entity,
        remaining_life,
    } in er.iter()
    {
        deaths += 1;
        info!(
            "A death has occurred with {} life left",
            remaining_life.unwrap_or_default()
        );
        commands.entity(*entity).despawn_recursive();
    }
    if deaths > 0 && creeps.iter().count() <= deaths {
        level_state
            .set(LevelState::Building)
            .map_err(|err| error!("Failed to set level state to building: {}", err))
            .ok();
    }
}

fn projectile_hit(
    mut er: EventReader<ProjectileHit>,
    mut ew: EventWriter<Death>,
    towers: Query<&Damage>,
    mut creeps: Query<&mut Life>,
) {
    for ProjectileHit(projectile) in er.iter() {
        if let Ok(mut life) = creeps.get_mut(projectile.target) {
            if let Ok(damage) = towers.get(projectile.origin) {
                let damage = match damage {
                    Damage::Range(range) => rand::thread_rng().gen_range(range.clone()),
                    Damage::Fixed(val) => *val,
                };
                info!("Creep {:?} took {} damage", projectile.target, damage);
                if life.0 >= damage {
                    life.0 -= damage;
                } else {
                    life.0 = 0;
                }

                if life.0 == 0 {
                    ew.send(Death {
                        remaining_life: None,
                        entity: projectile.target,
                    });
                }
            }
        }
    }
}

pub enum Type {
    Ground,
    Flying,
}
