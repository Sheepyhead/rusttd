use self::assets::GameState;
use crate::{towers::JustBuilt, workarounds::clear_input_events};
use bevy::prelude::{self, *};

pub mod assets;
pub mod map;

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::AppBuilder) {
        app.add_state(LevelState::Building)
            .add_system_set(
                SystemSet::on_enter(GameState::Play)
                    .with_system(map::build_ground.system().after("Build grid"))
                    .with_system(map::build_grid.system().label("Build grid")),
            )
            .add_system_set(
                SystemSet::on_update(LevelState::Building).with_system(build_five.system()),
            )
            .add_system_set(
                SystemSet::on_enter(LevelState::Choosing).with_system(clear_input_events.system()),
            )
            .add_system_set(
                SystemSet::on_update(LevelState::Choosing).with_system(choose_one.system()),
            );
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum LevelState {
    Building,
    Choosing,
    Spawning,
}

fn build_five(
    mut level_state: ResMut<State<LevelState>>,
    mut gem_count: Local<u32>,
    gems: Query<(), Added<JustBuilt>>,
) {
    for _ in gems.iter() {
        *gem_count += 1;
    }

    if *gem_count >= 5 {
        level_state
            .set(LevelState::Choosing)
            .map_err(|err| error!("Failed to set level state to Choosing: {}", err))
            .ok();
        *gem_count = 0;
    }
}

fn choose_one(mut level_state: ResMut<State<LevelState>>, gems: Query<(), With<JustBuilt>>) {
    if gems.iter().count() == 0 {
        level_state
            .set(LevelState::Spawning)
            .map_err(|err| error!("Failed to set level state to Spawning: {}", err))
            .ok();
    }
}
