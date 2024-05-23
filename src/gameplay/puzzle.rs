use crate::grid_math::{DirSet, Grid, Pos2};

use super::cell::Cell;

#[derive(serde::Serialize, serde:: Deserialize)]
pub struct Puzzle {
    grid: Grid<Cell>
}

impl Puzzle {
    pub fn new() -> Self {
        let mut grid = Grid::new();
        grid.insert((0, 0).into(), Cell::new(0, DirSet::ordered(false, false, true, true)));
        grid.insert((1, 1).into(), Cell::new(1, DirSet::ordered(false, true, false, true)));
        Self { grid }
    }

    pub fn cells(&self) -> impl Iterator<Item = (&Pos2, &Cell)> {
        self.grid.iter()
    }
}