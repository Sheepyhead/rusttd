use crate::assets::GameState;
use bevy::prelude::*;
use kurinji::Kurinji;
use std::fs::read_to_string;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Play).with_system(setup.system()));
    }
}

fn setup(mut kurinji: ResMut<Kurinji>) {
    let binding_ron = read_to_string("config/key_mappings.ron").unwrap();
    kurinji.set_bindings_with_ron(&binding_ron);
}
