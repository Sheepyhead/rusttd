use crate::{grid::Grid, maps::Ground};
use bevy::prelude::{shape::Plane, *};
use bevy_mod_picking::PickableBundle;

const BLOCKED_SLOTS: [(i32, i32); 28] = [
    (0, 1),
    (1, 1),
    (1, 2),
    (1, 12),
    (1, 13),
    (1, 14),
    (2, 1),
    (2, 13),
    (8, 1),
    (9, 1),
    (9, 2),
    (9, 20),
    (9, 21),
    (9, 22),
    (10, 1),
    (10, 21),
    (16, 1),
    (16, 13),
    (17, 0),
    (17, 1),
    (17, 2),
    (17, 12),
    (17, 13),
    (18, 13),
    (28, 21),
    (29, 21),
    (29, 22),
    (30, 21),
];

#[allow(clippy::cast_precision_loss)]
pub fn build_ground(
    mut commands: Commands,
    grid: Res<Grid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Plane { size: 100.0 }.into()),
            material: mats.add(Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(25.0, -0.1, 20.0)),
            ..PbrBundle::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert_bundle((Ground,));

    for (x, y) in grid.keys() {
        let new_pos = Vec3::new((*x as f32) - 0.5, -0.05, (*y as f32) - 0.5);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Plane { size: 1.0 }.into()),
            material: mats.add(Color::WHITE.into()),
            transform: Transform::from_translation(new_pos),
            ..PbrBundle::default()
        });
    }
}

/// # Panics
///
/// Will panic if grid fails to block properly, usually caused by the `Grid::clear` failing
pub fn build_grid(mut grid: ResMut<Grid>) {
    grid.clear();
    for (x, y) in BLOCKED_SLOTS {
        let x = x * 2;
        let y = y * 2;
        for x in x..=x + 1 {
            for y in y..=y + 1 {
                grid.block((x, y))
                    .unwrap_or_else(|_| panic!("Failed to block grid slot {};{}", x, y));
            }
        }
    }
}
