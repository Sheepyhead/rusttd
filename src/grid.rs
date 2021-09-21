use bevy::{
    math::Vec3Swizzles,
    prelude::{self, *},
    utils::HashMap,
};

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::AppBuilder) {
        app.insert_resource(Grid::default());
    }
}

pub enum Slot {
    Occupied(Entity),
    Blocked,
}

#[derive(Default)]
pub struct Grid(HashMap<(i32, i32), Slot>);

impl Grid {
    pub fn buildable(&self, pos: &[(i32, i32)]) -> bool {
        for pos in pos {
            if self.0.contains_key(pos) {
                return false;
            }
        }
        true
    }

    pub fn add_building(&mut self, pos: &[(i32, i32)], entity: Entity) -> Result<(), ()> {
        if self.buildable(pos) {
            for pos in pos {
                self.0.insert(*pos, Slot::Occupied(entity));
            }
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn snap_to_grid(pos: Vec3) -> Vec3 {
        Vec3::new(pos.x.round(), 0.0, pos.z.round())
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn to_grid_pos(pos: Vec3) -> (i32, i32) {
        let pos = Self::snap_to_grid(pos).xz();
        (pos.x as i32, pos.y as i32)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn block(&mut self, pos: (i32, i32)) -> Result<(), ()> {
        if self.buildable(&[pos]) {
            self.0.insert(pos, Slot::Blocked);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn keys(&self) -> std::collections::hash_map::Keys<(i32, i32), Slot> {
        self.0.keys()
    }
}
