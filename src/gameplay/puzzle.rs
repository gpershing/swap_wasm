use crate::grid_math::{DirSet, Grid};
use super::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub struct PuzzleCell {
    pub(crate) connections: DirSet,
    pub(crate) source: Option<Color>,
}

impl PuzzleCell {
    pub fn new(connections: DirSet, source: Option<Color>) -> Self {
        Self { connections, source }
    }

    pub fn nonsource(connections: DirSet) -> Self {
        Self { connections, source: None }
    }

    pub fn source(connections: DirSet, source: Color) -> Self {
        Self { connections, source: Some(source) }
    }
}

#[derive(serde::Serialize, serde:: Deserialize)]
pub struct Puzzle {
    grid: Grid<PuzzleCell>,
    swaps: u8
}

impl Puzzle {
    pub fn debug_default() -> Self {
        let mut grid = Grid::new();
        grid.insert((0, 0).into(), PuzzleCell::nonsource(DirSet::ordered(false, false, false, true)));
        grid.insert((0, 1).into(), PuzzleCell::source(DirSet::ordered(false, true, false, false), Color::Orange));
        grid.insert((1, 0).into(), PuzzleCell::nonsource(DirSet::ordered(false, false, false, true)));
        grid.insert((1, 1).into(), PuzzleCell::source(DirSet::ordered(false, true, false, false), Color::Purple));
        Self { grid, swaps: 4 }
    }

    pub fn new(grid: Grid<PuzzleCell>, swaps: u8) -> Self {
        Self { grid, swaps }
    }

    pub fn swap_limit(&self) -> u8 {
        self.swaps
    }

    pub fn start(&self) -> Grid<PuzzleCell> {
        self.grid.clone()
    }
}