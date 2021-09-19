use bevy::prelude::{shape::Plane, *};

pub struct Ground;

pub fn build(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Plane { size: 100.0 }.into()),
            ..PbrBundle::default()
        })
        .insert_bundle((Ground,));
}
