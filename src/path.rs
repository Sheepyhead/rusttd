use crate::grid::Grid;
use pathfinding::prelude::bfs;

pub fn resolve(grid: &Grid, goals: &[(i32, i32)]) -> Option<Vec<(i32, i32)>> {
    let mut res = vec![];
    for i in 1..goals.len() {
        let prev_goal = goals[i - 1];
        let goal = goals[i];
        let to_add = bfs(
            &prev_goal,
            |p| {
                let mut succ = grid.neighbours(*p);
                succ.retain(|x| match x {
                    // FIXME: map borders exist to avoid infinite search.
                    (-100..=100, -100..=100) => grid.get(*x).is_none(),
                    _ => false,
                });
                succ
            },
            |p| *p == goal,
        );
        res.append(&mut to_add?);
    }
    Some(res)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::grid::Grid;

    #[test]
    fn grid_pathfinding() {
        let mut grid = Grid::default();
        grid.clear();
        for (x, y) in [(2, 0), (3, 0)] {
            grid.block((x, y))
                .unwrap_or_else(|_| panic!("Failed to block grid slot {};{}", x, y));
        }
        let goal = (3, 0);
        let result = resolve(&grid, &[goal]);
        println!("result: {:?}", result);
    }
}
