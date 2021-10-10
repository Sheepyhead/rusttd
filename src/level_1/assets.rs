use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::AssetCollection;

#[derive(Clone, Component, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Loading,
    Play,
}

#[derive(AssetCollection, Component)]
pub struct Fonts {}

#[derive(AssetCollection, Component)]
pub struct Models {
    #[asset(path = "ps1wall.gltf")]
    pub wall: Handle<Gltf>,
}
