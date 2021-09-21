use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;
use kurinji::OnActionActive;

use crate::level_1::assets::GameState;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Play).with_system(setup.system()))
            .add_system_set(SystemSet::on_update(GameState::Play).with_system(control.system()));
    }
}

struct MainCamera;

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 30.0, 15.0))
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..PerspectiveCameraBundle::default()
        })
        .insert_bundle(PickingCameraBundle::default())
        .insert(MainCamera);

    commands.spawn_bundle(UiCameraBundle::default());
}

fn control(
    time: Res<Time>,
    mut er: EventReader<OnActionActive>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    let mut camera = camera.single_mut().expect("None or multiple main cameras!");
    let velocity = time.delta_seconds() * 10.0;
    for action in er.iter() {
        match action.action.as_str() {
            "CAMERA_RIGHT" => camera.translation.x += velocity,
            "CAMERA_LEFT" => camera.translation.x -= velocity,
            "CAMERA_DOWN" => camera.translation.y += velocity,
            "CAMERA_UP" => camera.translation.y -= velocity,
            "CAMERA_FORWARD" => camera.translation.z += velocity,
            "CAMERA_BACK" => camera.translation.z -= velocity,
            _ => continue,
        }
    }
}
