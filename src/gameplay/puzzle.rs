use crate::grids::{Direction, DirectionMap, DirectionSet, Grid, GridIndex, GridSize};

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

impl PuzzleCell {
    pub const fn source(&self) -> Option<Color> {
        match self {
            PuzzleCell::Normal { connections } => None,
            PuzzleCell::Source { connections, source } => Some(*source),
            PuzzleCell::Intersection { connections } => None,
        }
    }

    pub fn iter_layers(&self) -> impl Iterator<Item = DirectionSet> {
        match self {
            PuzzleCell::Normal { connections } => [Some(*connections), None],
            PuzzleCell::Source { connections, source: _source } => [Some(*connections), None],
            PuzzleCell::Intersection { connections } => {
                [Some(connections.map(|c| c == LayerConnection::Layer0)),
                Some(connections.map(|c| c == LayerConnection::Layer1))]
            }
        }.into_iter().filter_map(std::convert::identity)
    }

    pub fn total_connections(&self) -> usize {
        match self {
            PuzzleCell::Normal { connections } => connections.len(),
            PuzzleCell::Source { connections, source: _source } => connections.len(),
            PuzzleCell::Intersection { connections } => connections.iter().filter(|c| *c.1 != LayerConnection::None).count(),
        }
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

pub fn fallback_puzzle() -> Puzzle {
    let mut grid = Grid::with_size(GridSize { width: 2, height: 1});
    grid.insert(GridIndex { x: 0, y: 0 }, PuzzleCell::Source {
        connections: DirectionSet::from_iter([Direction::W].into_iter()),
        source: Color::SWAP }).unwrap();
    grid.insert(GridIndex { x: 1, y: 0 }, PuzzleCell::Normal {
        connections: DirectionSet::from_iter([Direction::E].into_iter()) })
        .unwrap();
    Puzzle { grid, swaps: 1 }
}