use crate::{
    abilities::OnHitAbilities,
    grid::Grid,
    level_1::{
        map::{self, get_creep},
        LevelState,
    },
    maps::Level,
    math_utils,
    path::resolve,
    towers::{Damage, ProjectileHit},
};
use bevy::prelude::{self, *};
use rand::Rng;

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
    pub life: Life,
    pub movement: Movement,
    pub speed: Speed,
    pub r#type: Type,
}

pub struct Speed {
    pub base: f32,
    pub min: f32,
    pub modifier: u32,
}

impl Speed {
    #[allow(clippy::cast_precision_loss)] // Allowed since modifier will never exceed the f32 mantissa
    pub fn total(&self) -> f32 {
        let modified = self.base - (((self.modifier as f32) / 100.0) * self.base);
        self.min.max(modified)
    }

    /// Reduces speed by the given percent modifier, for instance if modifier = 25 the movement speed is reduced by 25%
    pub fn reduce(&mut self, modifier: u32) {
        self.modifier += modifier;
    }

    /// Reverse operation of `Speed::reduce`, may fail if not temporally following a corresponding `Speed::reduce` since modifier is currently unsigned
    pub fn increase(&mut self, modifier: u32) {
        self.modifier -= modifier;
    }
}

pub struct Life(pub u64);

pub struct Movement {
    pub route: Vec<(i32, i32)>,
    pub destination: usize,
}

fn spawn(
    mut commands: Commands,
    time: Res<Time>,
    grid: Res<Grid>,
    level: Res<Level>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut spawners: Query<(Entity, &Transform, &mut Spawner)>,
) {
    for (spawner_entity, transform, mut spawner) in spawners.iter_mut() {
        spawner.timer.tick(time.delta());

        if spawner.timer.just_finished() {
            let mut creep = get_creep(level.0);

            // Only use pathfinding for ground creeps
            if let Type::Ground = creep.r#type {
                let mut route_for_spawner = vec![Grid::to_grid_pos(transform.translation / 2.0)];
                route_for_spawner.extend(creep.movement.route);
                creep.movement.route = resolve(&*grid, &route_for_spawner)
                    .unwrap_or_else(|| map::CREEP_ROUTE.to_vec());
            }

            commands.spawn_bundle(creep).insert_bundle(PbrBundle {
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
                commands.entity(spawner_entity).despawn_recursive();
            }
        }
    }
}

#[allow(clippy::cast_precision_loss)]
fn moving(
    time: Res<Time>,
    mut ew: EventWriter<Death>,
    mut creeps: Query<(Entity, &mut Transform, &mut Movement, &Life, &Speed)>,
) {
    for (creep_entity, mut transform, mut movement, Life(life), speed) in creeps.iter_mut() {
        if let Some(destination) = movement.route.get(movement.destination) {
            let speed = speed.total() * time.delta_seconds();

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

            ew.send(Death {
                remaining_life: Some(*life),
                entity: creep_entity,
            });
        }
    }
}

pub struct Death {
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
        remaining_life: _remaining_life,
    } in er.iter()
    {
        deaths += 1;

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
    mut commands: Commands,
    mut er: EventReader<ProjectileHit>,
    mut ew: EventWriter<Death>,
    towers: Query<(&Damage, &OnHitAbilities)>,
    mut creeps: Query<(Entity, &GlobalTransform, &mut Life)>,
) {
    for ProjectileHit(projectile) in er.iter() {
        if let Ok((target, position, mut life)) = creeps.get_mut(projectile.target) {
            if let Ok((damage, OnHitAbilities(abilities))) = towers.get(projectile.origin) {
                let mut damage = match damage {
                    Damage::Range(range) => rand::thread_rng().gen_range(range.clone()),
                    Damage::Fixed(val) => *val,
                };
                for on_hit in abilities {
                    on_hit.apply(target, &mut commands, &mut damage, position.translation);
                }
                damage_creep(projectile.target, damage, &mut life, &mut ew);
            }
        }
    }
}

pub fn damage_creep(target: Entity, damage: u64, mut life: &mut Life, ew: &mut EventWriter<Death>) {
    if life.0 >= damage {
        life.0 -= damage;
    } else {
        life.0 = 0;
    }
    if life.0 == 0 {
        ew.send(Death {
            remaining_life: None,
            entity: target,
        });
    }
}

#[derive(Clone, Copy)]
pub enum Type {
    Ground,
    Flying,
}
