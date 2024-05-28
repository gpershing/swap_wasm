use crate::grid_math::{DirSet, Grid};
use super::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub enum PuzzleCell {
    Normal {
        connections: DirSet
    },
    Source {
        connections: DirSet,
        source: Color
    },
    Intersection {
        layers: [DirSet; 2]
    }
}

#[derive(serde::Serialize, serde:: Deserialize)]
pub struct Puzzle {
    grid: Grid<PuzzleCell>,
    swaps: u8
}

impl Puzzle {
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