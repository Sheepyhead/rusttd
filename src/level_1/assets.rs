use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(Clone, Component, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Loading,
    Play,
}

#[derive(AssetCollection, Component)]
pub struct Fonts {}
