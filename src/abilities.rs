use self::on_hit::{OnHit, SplashEffect};
use crate::{
    creeps::{self, Death, Life},
    towers::{Damage, Range},
};
use bevy::prelude::{self, *};

pub mod on_hit;

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::AppBuilder) {
        app.add_system(splash.system());
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct OnHitAbilities(pub Vec<OnHit>);

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
fn splash(
    mut commands: Commands,
    mut ew: EventWriter<Death>,
    splash: Query<(Entity, &GlobalTransform, &SplashEffect, &Range, &Damage)>,
    mut creeps: Query<(Entity, &mut Life, &GlobalTransform), With<creeps::Type>>,
) {
    for (entity, transform, effect, Range(range), damage) in splash.iter() {
        for (entity, mut life, creep_position) in creeps.iter_mut() {
            // Check if within range's bounding box just to avoid the expensive sqrt in distance calc
            if creep_position.translation.x >= transform.translation.x - (*range / 2.0)
                && creep_position.translation.x <= transform.translation.x + (*range / 2.0)
                && creep_position.translation.z >= transform.translation.z - (*range / 2.0)
                && creep_position.translation.z <= transform.translation.z + (*range / 2.0)
            {
                // Now check all within bounding box to make splash circular rather than square
                if creep_position.translation.distance(transform.translation) <= *range {
                    creeps::damage_creep(
                        entity,
                        match effect {
                            SplashEffect::Multiplier { multiplier } => {
                                (match damage {
                                    Damage::Range(_) => unimplemented!(
                                        "No implementation for rolling random splash damage"
                                    ),
                                    Damage::Fixed(damage) => *damage,
                                } as f32
                                    * *multiplier) as u64
                            }
                        },
                        &mut life,
                        &mut ew,
                    );
                }
            }
        }
        commands.entity(entity).despawn_recursive();
    }
}
