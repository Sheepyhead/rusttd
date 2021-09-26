use bevy::{app::Events, prelude::*};
use kurinji::OnActionBegin;

pub fn clear_input_events(mut on_begin: ResMut<Events<OnActionBegin>>) {
    on_begin.clear();
}
