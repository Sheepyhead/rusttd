use super::{
    cooldown_is_done, launch_projectile, AttackSpeed, Cooldown, Gem, GemQuality, GemType, Range,
    TowerBundle, BASE_TOWER_SPEED,
};
use crate::{
    abilities::{aura::Auras, on_hit::OnHit, OnHitAbilities},
    creeps::Speed,
    level_1::LevelState,
    towers::{Damage, Target},
};
use bevy::prelude::{self, *};

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(LevelState::Spawning)
                .with_system(attack.system())
                .with_system(Slowed::added.system())
                .with_system(Slowed::system.system()),
        );
    }
}

fn attack(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gems: Query<(
        Entity,
        &GlobalTransform,
        &Gem,
        &AttackSpeed,
        &Target,
        &mut Cooldown,
    )>,
) {
    for (gem_entity, gem_position, gem, AttackSpeed(speed), Target(target), mut cooldown) in
        gems.iter_mut()
    {
        if !matches!(gem.r#type, GemType::Sapphire) {
            continue;
        }

        if !cooldown_is_done(&mut *cooldown, *speed, &time) {
            continue;
        }

        if let Some(target) = target {
            cooldown.0.reset();

            launch_projectile(
                &mut commands,
                &mut meshes,
                gem_position,
                gem_entity,
                *target,
            );
        }
    }
}

pub fn tower(quality: GemQuality) -> TowerBundle {
    match quality {
        GemQuality::Chipped => TowerBundle {
            damage: Damage::Range(5..=8),
            speed: AttackSpeed(BASE_TOWER_SPEED - 0.2),
            range: Range(5.5),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![OnHit::SapphireSlow(20)]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Flawed => TowerBundle {
            damage: Damage::Range(10..=14),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(7.5),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![OnHit::SapphireSlow(25)]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Normal => TowerBundle {
            damage: Damage::Range(16..=22),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(8.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![OnHit::SapphireSlow(30)]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Flawless => TowerBundle {
            damage: Damage::Range(30..=40),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(8.5),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![OnHit::SapphireSlow(35)]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
        GemQuality::Perfect => TowerBundle {
            damage: Damage::Range(60..=80),
            speed: AttackSpeed(BASE_TOWER_SPEED),
            range: Range(14.0),
            cooldown: Cooldown(Timer::from_seconds(1.0, true)),
            abilities: OnHitAbilities(vec![OnHit::SapphireSlow(40)]),
            auras: Auras(vec![]),
            target: Target::default(),
        },
    }
}

pub struct Slowed(pub u32, pub Timer);

impl Slowed {
    pub fn added(mut slowed_creeps: Query<(&Slowed, &mut Speed), Added<Slowed>>) {
        for (slowed, mut speed) in slowed_creeps.iter_mut() {
            speed.reduce(slowed.0);
        }
    }
    pub fn system(
        mut commands: Commands,
        time: Res<Time>,
        mut slowed_creeps: Query<(Entity, &mut Slowed, &mut Speed)>,
    ) {
        for (entity, mut slowed, mut speed) in slowed_creeps.iter_mut() {
            slowed.1.tick(time.delta());
            if slowed.1.just_finished() {
                commands.entity(entity).remove::<Slowed>();
                speed.increase(slowed.0);
            }
        }
    }
}
