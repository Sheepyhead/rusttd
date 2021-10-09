use crate::{
    buffs::{self, AttackSpeed},
    towers::{get_all_towers_within_range, Range, Tower},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Auras(pub Vec<Aura>);

pub struct Aura {
    pub range: Range,
    pub kind: Kind,
}

pub enum Kind {
    AttackSpeed(f32),
}

impl Auras {
    pub fn apply_new_aura(
        mut commands: Commands,
        auras: Query<(&Transform, &Auras), Added<Auras>>,
        towers: Query<(Entity, &GlobalTransform, Option<&buffs::AttackSpeed>), With<Tower>>,
    ) {
        for (transform, Auras(auras)) in auras.iter() {
            let position = transform.translation;
            for Aura { range, kind } in auras {
                for tower in get_all_towers_within_range(&towers, &position, *range) {
                    match kind {
                        Kind::AttackSpeed(val) => {
                            if let Ok(buffs::AttackSpeed(existing_buff)) =
                                towers.get_component::<AttackSpeed>(tower)
                            {
                                if existing_buff < val {
                                    commands.entity(tower).insert(buffs::AttackSpeed(*val));
                                }
                            } else {
                                commands.entity(tower).insert(buffs::AttackSpeed(*val));
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn apply_aura_to_new_tower(
        mut commands: Commands,
        auras: Query<(&Transform, &Auras)>,
        towers: Query<(Entity, &GlobalTransform, Option<&buffs::AttackSpeed>), Added<Tower>>,
    ) {
        for (tower, tower_position, attack_speed) in towers.iter() {
            for (aura_position, Auras(auras)) in auras.iter() {
                for Aura { range, kind } in auras.iter() {
                    if range.within(tower_position.translation, aura_position.translation) {
                        match kind {
                            Kind::AttackSpeed(val) => {
                                if let Some(buffs::AttackSpeed(existing_buff)) = attack_speed {
                                    if existing_buff < val {
                                        commands.entity(tower).insert(buffs::AttackSpeed(*val));
                                    }
                                } else {
                                    commands.entity(tower).insert(buffs::AttackSpeed(*val));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
