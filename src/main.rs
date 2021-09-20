#![warn(clippy::pedantic)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::enum_glob_use)]

use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use bevy_inspector_egui::InspectableRegistry;
use bevy_mod_picking::{InteractablePickingPlugin, PickingPlugin};
use bevy_rapier3d::{
    physics::{NoUserData, RapierPhysicsPlugin},
    render::RapierRenderPlugin,
};
use kurinji::KurinjiPlugin;
use level_1::assets::{self, GameState};

mod camera;
mod grid;
pub mod level_1;

fn main() {
    let mut app = App::build();

    AssetLoader::new(GameState::Loading, GameState::Play)
        .with_collection::<assets::Fonts>()
        .build(&mut app);

    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(InspectableRegistry::default())
        .insert_resource(ClearColor(Color::BLACK))
        // External plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(KurinjiPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        // Internal plugins
        .add_state(GameState::Loading)
        .add_plugin(level_1::Plugin)
        .add_plugin(camera::Plugin)
        .add_plugin(grid::Plugin)
        .run();
}
