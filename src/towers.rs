use crate::{grid::Grid, level_1::LevelState};
use bevy::prelude::{self, *};

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::AppBuilder) {
        app.add_event::<BuildGem>()
            .add_event::<ChooseGem>()
            .add_system_set(
                SystemSet::on_update(LevelState::Building).with_system(build_gem.system()),
            )
            .add_system_set(
                SystemSet::on_update(LevelState::Choosing).with_system(choose_gem.system()),
            );
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
                },
                JustBuilt,
            ))
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
