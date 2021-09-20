use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;

use crate::level_1::assets::GameState;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Play).with_system(setup.system()));
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 30.0, 15.0))
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..PerspectiveCameraBundle::default()
        })
        .insert_bundle(PickingCameraBundle::default());

    commands.spawn_bundle(UiCameraBundle::default());
}
