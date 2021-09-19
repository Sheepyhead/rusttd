use bevy::prelude::{shape::Plane, *};

pub struct Ground;

pub fn build(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Plane { size: 100.0 }.into()),
            material: mats.add(Color::DARK_GREEN.into()),
            ..PbrBundle::default()
        })
        .insert_bundle((Ground,));
}
