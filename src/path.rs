use crate::grid::Grid;
use pathfinding::prelude::bfs;

pub fn resolve(grid: &Grid, goals: &[(i32, i32)]) -> Option<Vec<(i32, i32)>> {
    let mut res = vec![];
    for i in 1..goals.len() {
        let prev_goal = goals[i - 1];
        let goal = goals[i];
        let to_add = bfs(
            &prev_goal,
            |start| {
                let mut succ = Grid::neighbours(*start);
                succ.retain(|goal| match goal {
                    // FIXME: map borders exist to avoid infinite search.
                    (-100..=100, -100..=100) => grid.get(*goal).is_none(),
                    _ => false,
                });
                succ
            },
            |start| *start == goal,
        );
        res.append(&mut to_add?);
    }
    Some(res)
}
