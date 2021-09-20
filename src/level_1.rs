use self::assets::GameState;
use bevy::prelude::{self, *};

pub mod assets;
pub mod map;

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Play)
                .with_system(map::build_ground.system())
                .with_system(map::build_grid.system()),
        );
    }
}
