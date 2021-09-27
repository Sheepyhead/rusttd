use crate::{grid::Grid, level_1::LevelState};
use bevy::prelude::{self, *};

mod diamond;

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::AppBuilder) {
        app.add_plugin(diamond::Plugin)
            .add_event::<BuildGem>()
            .add_event::<ChooseGem>()
            .add_event::<ProjectileHit>()
            .add_system_set(
                SystemSet::on_update(LevelState::Building).with_system(build_gem.system()),
            )
            .add_system_set(
                SystemSet::on_update(LevelState::Choosing).with_system(choose_gem.system()),
            )
            .add_system(move_projectile.system());
    }
}

pub enum GemQuality {
    Chipped,
}

pub enum GemType {
    Diamond,
}

pub struct Gem {
    pub quality: GemQuality,
    pub r#type: GemType,
    pub cooldown: Timer,
}

pub struct JustBuilt;

pub struct BuildGem {
    pub pos: (i32, i32),
}

#[allow(clippy::cast_precision_loss)]
fn build_gem(
    mut commands: Commands,
    mut er: EventReader<BuildGem>,
    mut grid: ResMut<Grid>,
    mut meshes: ResMut<Assets<Mesh>>,
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

        let entity = commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(shape::Cube::new(2.0).into()),
                transform: Transform::from_translation(Vec3::new(pos.0 as f32, 0.5, pos.1 as f32)),
                ..PbrBundle::default()
            })
            .insert_bundle((
                Gem {
                    quality: GemQuality::Chipped,
                    r#type: GemType::Diamond,
                    cooldown: Timer::from_seconds(1.0, true),
                },
                JustBuilt,
            ))
            .insert_bundle(TowerBundle {
                damage: Damage(20),
                speed: AttackSpeed(80),
                range: Range(5.0),
            })
            .id();
        grid.add_building(&positions, entity)
            .map_err(|_| info!("Failed to add building to {};{}", pos.0, pos.1))
            .ok();
    }
}

pub struct ChooseGem {
    pub pos: (i32, i32),
}

pub struct Rock;

fn choose_gem(
    mut commands: Commands,
    mut er: EventReader<ChooseGem>,
    grid: ResMut<Grid>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut gems: Query<(Entity, &mut Handle<StandardMaterial>), (With<Gem>, With<JustBuilt>)>,
) {
    for ChooseGem { pos } in er.iter() {
        if let Some(chosen_entity) = grid.get(*pos) {
            if gems.get_mut(chosen_entity).is_err() {
                continue;
            }

            for (entity, mut material) in gems.iter_mut() {
                if entity == chosen_entity {
                    *material = mats.add(Color::WHITE.into());
                } else {
                    *material = mats.add(Color::DARK_GRAY.into());
                    commands.entity(entity).remove::<Gem>().insert(Rock);
                }
                commands.entity(entity).remove::<JustBuilt>();
            }
        }
    }
}

#[derive(Clone, Copy)]
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
        }
    }
}

pub struct Damage(u64);

pub struct AttackSpeed(u64);

pub struct Range(f32);

#[derive(Bundle)]
pub struct TowerBundle {
    damage: Damage,
    speed: AttackSpeed,
    range: Range,
}
