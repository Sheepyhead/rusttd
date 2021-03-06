use crate::{
    abilities::{aura::Auras, OnHitAbilities},
    buffs, creeps,
    grid::Grid,
    level_1::LevelState,
};
use bevy::prelude::{self, *};
use rand::{
    distributions::Standard,
    prelude::{Distribution, IteratorRandom},
};
use std::{ops::RangeInclusive, time::Duration};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// Most towers attack at a base rate of 1 attack per second
pub const BASE_TOWER_SPEED: f32 = 1.0;

mod amethyst;
mod aquamarine;
mod diamond;
pub mod emerald;
mod opal;
mod ruby;
pub mod sapphire;
mod topaz;

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::App) {
        app.add_plugin(diamond::Plugin)
            .add_plugin(aquamarine::Plugin)
            .add_plugin(amethyst::Plugin)
            .add_plugin(emerald::Plugin)
            .add_plugin(opal::Plugin)
            .add_plugin(ruby::Plugin)
            .add_plugin(sapphire::Plugin)
            .add_plugin(topaz::Plugin)
            .add_event::<BuildGem>()
            .add_event::<ChooseGem>()
            .add_event::<ProjectileHit>()
            .insert_resource(RangeDisplay::Off)
            .add_system(render_range)
            .add_system_set(SystemSet::on_update(LevelState::Building).with_system(build_gem))
            .add_system_set(SystemSet::on_enter(LevelState::Choosing).with_system(reveal_gems))
            .add_system_set(SystemSet::on_update(LevelState::Choosing).with_system(choose_gem))
            .add_system_set(
                SystemSet::on_exit(LevelState::Choosing).with_system(despawn_range_render),
            )
            .add_system(move_projectile)
            .add_system(pick_target);
    }
}

#[derive(Clone, Copy, EnumIter)]
pub enum GemQuality {
    Chipped,
    Flawed,
    Normal,
    Flawless,
    Perfect,
}

impl Distribution<GemQuality> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> GemQuality {
        GemQuality::iter().choose(rng).unwrap()
    }
}

#[derive(Clone, Copy, EnumIter)]
pub enum GemType {
    Diamond,
    Aquamarine,
    Ruby,
    Emerald,
    Sapphire,
    Topaz,
    Opal,
    Amethyst,
}

impl GemType {
    pub fn color(&self) -> Color {
        match self {
            GemType::Diamond => Color::WHITE,
            GemType::Aquamarine => Color::AQUAMARINE,
            GemType::Ruby => Color::RED,
            GemType::Emerald => Color::GREEN,
            GemType::Sapphire => Color::BLUE,
            GemType::Topaz => Color::YELLOW,
            GemType::Opal => Color::PURPLE,
            GemType::Amethyst => Color::PINK,
        }
    }
}

impl Distribution<GemType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> GemType {
        GemType::iter().choose(rng).unwrap()
    }
}

#[derive(Component)]
pub struct Gem {
    pub quality: GemQuality,
    pub r#type: GemType,
}

impl Gem {
    fn _shape(&self) -> shape::Cube {
        shape::Cube {
            size: match self.quality {
                GemQuality::Chipped => 0.4,
                GemQuality::Flawed => 0.8,
                GemQuality::Normal => 1.2,
                GemQuality::Flawless => 1.6,
                GemQuality::Perfect => 2.0,
            },
        }
    }

    pub fn tower(&self) -> TowerBundle {
        match self.r#type {
            GemType::Diamond => diamond::tower(self.quality),
            GemType::Aquamarine => aquamarine::tower(self.quality),
            GemType::Ruby => ruby::tower(self.quality),
            GemType::Emerald => emerald::tower(self.quality),
            GemType::Sapphire => sapphire::tower(self.quality),
            GemType::Topaz => topaz::tower(self.quality),
            GemType::Opal => opal::tower(self.quality),
            GemType::Amethyst => amethyst::tower(self.quality),
        }
    }
}

#[derive(Component)]
pub struct JustBuilt;

pub struct BuildGem {
    pub pos: (i32, i32),
}

#[allow(clippy::cast_precision_loss)]
fn build_gem(
    mut commands: Commands,
    mut er: EventReader<BuildGem>,
    ass: ResMut<AssetServer>,
    mut grid: ResMut<Grid>,
) {
    for BuildGem { pos } in er.iter() {
        let positions = [
            *pos,
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0 + 1, pos.1 + 1),
        ];

        if !grid.buildable(&positions) {
            continue;
        }

        let mesh: Handle<Mesh> = ass.load("hole.gltf#Mesh0/Primitive0");
        let material: Handle<StandardMaterial> = ass.load("hole.gltf#Material0");
        let entity = commands
            .spawn_bundle(PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(Vec3::new(pos.0 as f32, 0.5, pos.1 as f32)),
                ..PbrBundle::default()
            })
            .insert(JustBuilt)
            .id();
        grid.add_building(&positions, entity)
            .map_err(|_| info!("Failed to add building to {};{}", pos.0, pos.1))
            .ok();
    }
}

pub struct ChooseGem {
    pub pos: (i32, i32),
}

fn reveal_gems(
    mut commands: Commands,
    ass: ResMut<AssetServer>,
    mut gems: Query<Entity, With<JustBuilt>>,
) {
    for entity in gems.iter_mut() {
        let r#type: GemType = rand::random();
        let quality: GemQuality = rand::random();
        let gem = Gem { quality, r#type };
        let mesh: Handle<Mesh> = ass.load("clearcube.gltf#Mesh0/Primitive0");
        let material: Handle<StandardMaterial> = ass.load("clearcube.gltf#Material0");
        commands
            .entity(entity)
            .insert_bundle(gem.tower())
            .insert_bundle((gem, Tower, mesh, material));
    }
}

#[derive(Component)]
pub struct Rock;

fn choose_gem(
    mut commands: Commands,
    mut er: EventReader<ChooseGem>,
    grid: ResMut<Grid>,
    ass: ResMut<AssetServer>,
    mut gems: Query<Entity, With<JustBuilt>>,
) {
    for ChooseGem { pos } in er.iter() {
        if let Some(chosen_entity) = grid.get(*pos) {
            if gems.get_mut(chosen_entity).is_err() {
                continue;
            }

            for entity in gems.iter_mut() {
                if entity != chosen_entity {
                    let mesh: Handle<Mesh> = ass.load("ps1wall.gltf#Mesh0/Primitive0");
                    let mat: Handle<StandardMaterial> = ass.load("ps1wall.gltf#Material0");
                    commands
                        .entity(entity)
                        .remove::<Gem>()
                        .remove_bundle::<TowerBundle>()
                        .remove::<Tower>()
                        .insert_bundle((Rock, mesh, mat));
                }
                commands.entity(entity).remove::<JustBuilt>();
            }
        }
    }
}

#[derive(Clone, Component, Copy)]
pub struct Projectile {
    pub origin: Entity,
    pub target: Entity,
}

pub struct ProjectileHit(pub Projectile);

fn move_projectile(
    mut commands: Commands,
    time: Res<Time>,
    mut ew: EventWriter<ProjectileHit>,
    mut projectile: Query<(Entity, &mut Transform, &Projectile)>,
    positions: Query<&GlobalTransform>,
) {
    for (proj_entity, mut transform, projectile) in projectile.iter_mut() {
        let target = positions.get(projectile.target);
        if let Ok(target) = target {
            let mut direction = target.translation - transform.translation;
            direction = direction.normalize();
            direction *= 10.0 * time.delta_seconds();
            transform.translation += direction;

            if (target.translation.x - transform.translation.x).abs() <= 0.05
                && (target.translation.z - transform.translation.z).abs() <= 0.05
            {
                ew.send(ProjectileHit(*projectile));
                commands.entity(proj_entity).despawn_recursive();
            }
        } else {
            commands.entity(proj_entity).despawn_recursive();
        }
    }
}

#[derive(Component)]
pub enum Damage {
    Range(RangeInclusive<u64>),
    Fixed(u64),
}

#[derive(Component)]
pub struct AttackSpeed(pub f32);

#[derive(Clone, Component, Copy)]
pub struct Range(pub f32);

#[derive(Component)]
pub struct Cooldown(Timer);

#[derive(Component, Default)]
pub struct Target(Option<Entity>);

#[derive(Component)]
pub struct Tower;

#[derive(Bundle)]
pub struct TowerBundle {
    name: Name,
    damage: Damage,
    speed: AttackSpeed,
    range: Range,
    cooldown: Cooldown,
    abilities: OnHitAbilities,
    auras: Auras,
    target: Target,
}

fn launch_projectile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    gem_position: &GlobalTransform,
    gem_entity: Entity,
    closest_creep: Entity,
) {
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

fn cooldown_is_done(cooldown: &mut Cooldown, speed: f32, time: &Time) -> bool {
    cooldown
        .0
        .set_duration(Duration::from_secs_f32(1.0 * speed));
    cooldown.0.tick(time.delta());
    cooldown.0.finished()
}

fn get_closest_creep_within_range(
    creeps: &Query<(Entity, &GlobalTransform, &creeps::Type)>,
    tower_position: &GlobalTransform,
    range: f32,
    filter: Option<creeps::Type>,
) -> Option<Entity> {
    let mut closest = None;
    let mut closest_distance = f32::INFINITY;
    for (creep, position, r#type) in creeps.iter() {
        if let Some(filter) = filter {
            match (filter, *r#type) {
                (creeps::Type::Ground, creeps::Type::Ground)
                | (creeps::Type::Flying, creeps::Type::Flying) => {}
                _ => continue,
            }
        }
        let distance = tower_position
            .translation
            .distance_squared(position.translation);

        if distance < closest_distance {
            closest = Some(creep);
            closest_distance = distance;
        }
    }

    if closest_distance >= range.powf(2.0) {
        return None;
    }

    closest
}

fn get_all_creeps_within_range(
    creeps: &Query<(Entity, &GlobalTransform, &creeps::Type)>,
    tower_position: &GlobalTransform,
    range: Range,
    filter: Option<creeps::Type>,
) -> Vec<Entity> {
    creeps
        .iter()
        .filter_map(|(entity, transform, r#type)| {
            if range.within(transform.translation, tower_position.translation) {
                filter.map_or(Some(entity), |filter| match (filter, *r#type) {
                    (creeps::Type::Ground, creeps::Type::Ground)
                    | (creeps::Type::Flying, creeps::Type::Flying) => Some(entity),
                    _ => None,
                })
            } else {
                None
            }
        })
        .collect()
}

pub fn get_all_towers_within_range(
    towers: &Query<(Entity, &GlobalTransform, Option<&buffs::AttackSpeed>), With<Tower>>,
    tower_position: &Vec3,
    range: Range,
) -> Vec<Entity> {
    towers
        .iter()
        .filter_map(|(entity, transform, _)| {
            if range.within(transform.translation, *tower_position) {
                Some(entity)
            } else {
                None
            }
        })
        .collect()
}

#[allow(dead_code)]
enum RangeDisplay {
    Off,
    On(Color),
}

#[derive(Component)]
struct RangeVisualization(Entity);

fn render_range(
    mut commands: Commands,
    display: Res<RangeDisplay>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    towers: Query<(Entity, &GlobalTransform, &Range), Added<Range>>,
) {
    if let RangeDisplay::On(color) = *display {
        let color = Color::rgba(color.r(), color.g(), color.b(), 0.1);
        for (entity, transform, Range(range)) in towers.iter() {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(
                        shape::Icosphere {
                            radius: *range,
                            subdivisions: 30,
                        }
                        .into(),
                    ),
                    material: mats.add(color.into()),
                    visible: Visible {
                        is_transparent: true,
                        ..Visible::default()
                    },
                    transform: Transform::from_translation(transform.translation),
                    ..PbrBundle::default()
                })
                .insert(RangeVisualization(entity));
        }
    }
}

fn despawn_range_render(
    mut commands: Commands,
    removed: Query<Entity, With<Rock>>,
    ranges: Query<(Entity, &RangeVisualization)>,
) {
    for removed_entity in removed.iter() {
        for (entity, range) in ranges.iter() {
            if removed_entity == range.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

impl Range {
    pub fn within(&self, target: Vec3, origin: Vec3) -> bool {
        // Check if within range's bounding box just to avoid the expensive sqrt in distance calc
        (target.x >= origin.x - (self.0 / 2.0)
            && target.x <= origin.x + (self.0 / 2.0)
            && target.z >= origin.z - (self.0 / 2.0)
            && target.z <= origin.z + (self.0 / 2.0))
            && (
                // Now check all within bounding box to make splash circular rather than square
                target.distance(origin) <= self.0
            )
    }
}

fn pick_target(
    mut towers: Query<(&GlobalTransform, &Range, &mut Target), Without<creeps::Type>>,
    creeps: Query<(Entity, &GlobalTransform, &creeps::Type)>,
) {
    for (tower_pos, range, mut target) in towers.iter_mut() {
        if let Some(target_entity) = target.0 {
            if let Ok(creep_pos) = creeps.get_component::<GlobalTransform>(target_entity) {
                if !range.within(creep_pos.translation, tower_pos.translation) {
                    target.0 = None;
                }
            } else {
                target.0 = None;
            }
        } else {
            target.0 = get_closest_creep_within_range(&creeps, tower_pos, range.0, None);
        }
    }
}
