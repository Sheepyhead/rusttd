use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Loading,
    Play,
}

#[derive(AssetCollection)]
pub struct Fonts {}
