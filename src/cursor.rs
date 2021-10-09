use crate::{
    grid,
    level_1::LevelState,
    maps::Ground,
    towers::{BuildGem, ChooseGem},
};
use bevy::prelude::{self, shape::Plane, *};
use bevy_mod_picking::PickingCamera;
use kurinji::OnActionBegin;

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::App) {
        app.insert_resource(ShowGrid(true))
            .add_system_set(SystemSet::on_enter(LevelState::Building).with_system(activate_cursor))
            .add_system_set(
                SystemSet::on_update(LevelState::Building)
                    .with_system(render_grid)
                    .with_system(build_on_click),
            )
            .add_system_set(
                SystemSet::on_exit(LevelState::Building)
                    .with_system(deactivate_cursor.label("deactivate"))
                    .with_system(render_grid.after("deactivate")),
            )
            .add_system_set(
                SystemSet::on_update(LevelState::Choosing).with_system(choose_on_click),
            );
    }
}

#[derive(Default)]
pub struct ShowGrid(bool);

#[derive(Component)]
pub struct Grid;

fn activate_cursor(mut show: ResMut<ShowGrid>) {
    show.0 = true;
}

fn deactivate_cursor(mut show: ResMut<ShowGrid>) {
    show.0 = false;
}

fn render_grid(
    mut commands: Commands,
    show: Res<ShowGrid>,
    grid: Res<grid::Grid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cursors: Query<(Entity, &mut Transform, &mut Handle<StandardMaterial>), With<Grid>>,
    cameras: Query<&PickingCamera>,
    ground: Query<(), With<Ground>>,
) {
    if show.0 {
        let camera = cameras.single();

        let (picked_entity, intersection) = if let Some(val) = camera.intersect_top() {
            val
        } else {
            despawn_cursor(&mut cursors, &mut commands);
            return;
        };

        if ground.get(picked_entity).is_err() {
            despawn_cursor(&mut cursors, &mut commands);
            return;
        }

        let grid_pos = grid::Grid::snap_to_grid(intersection.position());

        let material = materials.add(
            if grid.buildable(&[
                grid::Grid::to_grid_pos(grid_pos),
                grid::Grid::to_grid_pos(Vec3::new(grid_pos.x + 1.0, grid_pos.y, grid_pos.z)),
                grid::Grid::to_grid_pos(Vec3::new(grid_pos.x, grid_pos.y, grid_pos.z + 1.0)),
                grid::Grid::to_grid_pos(Vec3::new(grid_pos.x + 1.0, grid_pos.y, grid_pos.z + 1.0)),
            ]) {
                Color::BLUE.into()
            } else {
                Color::RED.into()
            },
        );

        if let Ok((_, mut transform, mut mat)) = cursors.get_single_mut() {
            if transform.translation != grid_pos {
                transform.translation = grid_pos;
            }
            if *mat != material {
                *mat = material;
            }
        } else {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Plane { size: 2.0 }.into()),
                    transform: Transform::from_translation(grid_pos),
                    material,
                    ..PbrBundle::default()
                })
                .insert(Grid);
        }
    } else {
        despawn_cursor(&mut cursors, &mut commands);
    }
}

fn despawn_cursor(
    cursors: &mut Query<(Entity, &mut Transform, &mut Handle<StandardMaterial>), With<Grid>>,
    commands: &mut Commands,
) {
    for (entity, _, _) in cursors.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

fn build_on_click(
    mut er: EventReader<OnActionBegin>,
    mut ew: EventWriter<BuildGem>,
    cameras: Query<&PickingCamera>,
    ground: Query<(), With<Ground>>,
) {
    for action in er.iter() {
        if action.action != "LEFT_CLICK" {
            continue;
        }

        let camera = cameras.single();

        let (picked_entity, intersection) = if let Some(val) = camera.intersect_top() {
            val
        } else {
            continue;
        };

        if ground.get(picked_entity).is_err() {
            continue;
        }

        let grid_pos = grid::Grid::to_grid_pos(intersection.position());

        ew.send(BuildGem { pos: grid_pos });
    }
}

fn choose_on_click(
    mut er: EventReader<OnActionBegin>,
    mut ew: EventWriter<ChooseGem>,
    cameras: Query<&PickingCamera>,
    ground: Query<(), With<Ground>>,
) {
    for action in er.iter() {
        if action.action != "LEFT_CLICK" {
            continue;
        }
        let camera = cameras.single();

        let (picked_entity, intersection) = if let Some(val) = camera.intersect_top() {
            val
        } else {
            continue;
        };

        if ground.get(picked_entity).is_err() {
            continue;
        }

        let grid_pos = grid::Grid::to_grid_pos(intersection.position());

        ew.send(ChooseGem { pos: grid_pos });
    }
}
