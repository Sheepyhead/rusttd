use crate::{
    creeps::{self, CreepBundle, Life, Movement, Speed},
    grid::Grid,
    maps::Ground,
};
use bevy::prelude::{shape::Plane, *};
use bevy_mod_picking::PickableBundle;

const BLOCKED_SLOTS: [(i32, i32); 28] = [
    (0, 1),
    (1, 1),
    (1, 2),
    (1, 12),
    (1, 13),
    (1, 14),
    (2, 1),
    (2, 13),
    (8, 1),
    (9, 1),
    (9, 2),
    (9, 20),
    (9, 21),
    (9, 22),
    (10, 1),
    (10, 21),
    (16, 1),
    (16, 13),
    (17, 0),
    (17, 1),
    (17, 2),
    (17, 12),
    (17, 13),
    (18, 13),
    (28, 21),
    (29, 21),
    (29, 22),
    (30, 21),
];

pub const CREEP_ROUTE: [(i32, i32); 8] = [
    (2, 2),
    (2, 26),
    (34, 26),
    (34, 2),
    (18, 2),
    (18, 42),
    (58, 42),
    (58, 80),
];

#[allow(clippy::cast_precision_loss)]
pub fn build_ground(
    mut commands: Commands,
    grid: Res<Grid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Plane { size: 1000.0 }.into()),
            material: mats.add(Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(25.0, -0.1, 20.0)),
            ..PbrBundle::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert_bundle((Ground,));

    for (x, y) in grid.keys() {
        let new_pos = Vec3::new((*x as f32) - 0.5, -0.05, (*y as f32) - 0.5);
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Plane { size: 1.0 }.into()),
            material: mats.add(Color::WHITE.into()),
            transform: Transform::from_translation(new_pos),
            ..PbrBundle::default()
        });
    }
}

/// # Panics
///
/// Will panic if grid fails to block properly, usually caused by the `Grid::clear` failing
pub fn build_grid(mut grid: ResMut<Grid>) {
    grid.clear();
    for (x, y) in BLOCKED_SLOTS {
        let x = x * 2;
        let y = y * 2;
        for x in x..=x + 1 {
            for y in y..=y + 1 {
                grid.block((x, y))
                    .unwrap_or_else(|_| panic!("Failed to block grid slot {};{}", x, y));
            }
        }
    }
}

pub fn get_creep(level: u32) -> CreepBundle {
    let movement = Movement {
        route: CREEP_ROUTE.into(),
        destination: 0,
    };
    match level {
        1 => CreepBundle {
            life: Life(10),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        2 => CreepBundle {
            life: Life(30),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        3 => CreepBundle {
            life: Life(55),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        4 => CreepBundle {
            life: Life(70),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Flying,
        },
        5 => CreepBundle {
            life: Life(90),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        6 => CreepBundle {
            life: Life(120),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        7 => CreepBundle {
            life: Life(178),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        8 => CreepBundle {
            life: Life(240),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Flying,
        },
        9 => CreepBundle {
            life: Life(300),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        10 => CreepBundle {
            life: Life(470),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        11 => CreepBundle {
            life: Life(490),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        12 => CreepBundle {
            life: Life(450),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Flying,
        },
        13 => CreepBundle {
            life: Life(570),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        14 => CreepBundle {
            life: Life(650),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        15 => CreepBundle {
            life: Life(1000),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        16 => CreepBundle {
            life: Life(725),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Flying,
        },
        17 => CreepBundle {
            life: Life(1350),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        18 => CreepBundle {
            life: Life(1550),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        19 => CreepBundle {
            life: Life(1950),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        20 => CreepBundle {
            life: Life(1350),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Flying,
        },
        21 => CreepBundle {
            life: Life(2300),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        22 => CreepBundle {
            life: Life(2530),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        23 => CreepBundle {
            life: Life(3000),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        24 => CreepBundle {
            life: Life(2500),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Flying,
        },
        25 => CreepBundle {
            life: Life(3750),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        26 => CreepBundle {
            life: Life(4500),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        27 => CreepBundle {
            life: Life(5000),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        28 => CreepBundle {
            life: Life(4150),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Flying,
        },
        29 => CreepBundle {
            life: Life(6750),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        30 => CreepBundle {
            life: Life(7150),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        31 => CreepBundle {
            life: Life(8000),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        32 => CreepBundle {
            life: Life(6200),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Flying,
        },
        33 => CreepBundle {
            life: Life(9550),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        34 => CreepBundle {
            life: Life(10200),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        35 => CreepBundle {
            life: Life(11500),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        36 => CreepBundle {
            life: Life(8500),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Flying,
        },
        37 => CreepBundle {
            life: Life(13000),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        38 => CreepBundle {
            life: Life(15000),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        39 => CreepBundle {
            life: Life(17000),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        40 => CreepBundle {
            life: Life(10500),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Flying,
        },
        41 => CreepBundle {
            life: Life(19500),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
        _ => CreepBundle {
            life: Life(23000),
            movement,
            speed: Speed {
                base: 5.0,
                min: 0.2,
                modifier: 0,
            },
            r#type: creeps::Type::Ground,
        },
    }
}
