use self::on_hit::OnHit;
use bevy::prelude;

pub mod on_hit;

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, _: &mut prelude::AppBuilder) {
        // placeholder for future abilities that will need their own systems
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct OnHitAbilities(pub Vec<OnHit>);
