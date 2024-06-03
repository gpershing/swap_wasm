use crate::grids::{DirectionMap, DirectionSet, Grid};

use super::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub enum LayerConnection {
    #[default] None,
    Layer0,
    Layer1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub enum PuzzleCell {
    Normal {
        connections: DirectionSet
    },
    Source {
        connections: DirectionSet,
        source: Color
    },
    Intersection {
        connections: DirectionMap<LayerConnection>
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