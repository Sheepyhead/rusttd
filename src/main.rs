#![warn(clippy::pedantic)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::enum_glob_use)]

use bevy::{pbr::AmbientLight, prelude::*};
use bevy_asset_loader::AssetLoader;
use bevy_inspector_egui::InspectableRegistry;
use bevy_mod_picking::{InteractablePickingPlugin, PickingPlugin};
use kurinji::KurinjiPlugin;
use level_1::assets::{self, GameState};

mod abilities;
pub mod buffs;
mod camera;
pub mod creeps;
pub mod cursor;
mod grid;
mod input;
pub mod level_1;
pub mod maps;
pub mod math_utils;
pub mod path;
pub mod towers;
pub mod workarounds;

fn main() {
    let mut app = App::new();

    AssetLoader::new(GameState::Loading, GameState::Play)
        .with_collection::<assets::Fonts>()
        .build(&mut app);

    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(InspectableRegistry::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        })
        // External plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(KurinjiPlugin)
        // Internal plugins
        .add_state(GameState::Loading)
        .add_plugin(level_1::Plugin)
        .add_plugin(camera::Plugin)
        .add_plugin(grid::Plugin)
        .add_plugin(cursor::Plugin)
        .add_plugin(input::Plugin)
        .add_plugin(towers::Plugin)
        .add_plugin(creeps::Plugin)
        .add_plugin(abilities::Plugin)
        .add_plugin(buffs::Plugin)
        .run();
}
