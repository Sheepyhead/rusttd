use self::assets::GameState;
use crate::towers::Gem;
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
    gems: Query<(), Added<Gem>>,
) {
    for _ in gems.iter() {
        *gem_count += 1;
    }

    if *gem_count >= 5 {
        match level_state.set(LevelState::Choosing) {
            Ok(_) => {}
            Err(_) => todo!(),
        };
        *gem_count = 0;
    }
}
