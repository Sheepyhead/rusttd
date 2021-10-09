use crate::towers;
use bevy::prelude::{self, *};

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(AttackSpeed::apply);
    }
}

#[derive(Component, PartialEq, PartialOrd)]
pub struct AttackSpeed(pub f32);

impl AttackSpeed {
    fn apply(mut buff: Query<(&mut towers::AttackSpeed, &AttackSpeed), Added<AttackSpeed>>) {
        for (mut stat, AttackSpeed(buff)) in buff.iter_mut() {
            stat.0 += *buff;
        }
    }
}
