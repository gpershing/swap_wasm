use crate::grids::{Direction, DirectionMap, DirectionSet, Grid, GridIndex, GridSize};

use super::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde:: Deserialize)]
pub enum LayerConnection {
    #[default]
    None,
    Layer0,
    Layer1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde:: Deserialize)]
pub enum PuzzleCell {
    Normal {
        connections: DirectionSet,
    },
    Source {
        connections: DirectionSet,
        source: Color,
    },
    Intersection {
        connections: DirectionMap<LayerConnection>,
    },
}

impl PuzzleCell {
    pub const fn source(&self) -> Option<Color> {
        match self {
            PuzzleCell::Normal { connections: _ } => None,
            PuzzleCell::Source {
                connections: _,
                source,
            } => Some(*source),
            PuzzleCell::Intersection { connections: _ } => None,
        }
    }

    pub fn iter_layers(&self) -> impl Iterator<Item = DirectionSet> {
        match self {
            PuzzleCell::Normal { connections } => [Some(*connections), None],
            PuzzleCell::Source {
                connections,
                source: _source,
            } => [Some(*connections), None],
            PuzzleCell::Intersection { connections } => [
                Some(connections.map(|c| c == LayerConnection::Layer0)),
                Some(connections.map(|c| c == LayerConnection::Layer1)),
            ],
        }
        .into_iter()
        .flatten()
    }

    pub fn total_connections(&self) -> usize {
        match self {
            PuzzleCell::Normal { connections } => connections.len(),
            PuzzleCell::Source {
                connections,
                source: _source,
            } => connections.len(),
            PuzzleCell::Intersection { connections } => connections
                .iter()
                .filter(|c| *c.1 != LayerConnection::None)
                .count(),
        }
    }
}

#[derive(serde::Serialize, serde:: Deserialize, Debug, Clone)]
pub struct Puzzle {
    grid: Grid<PuzzleCell>,
    swaps: u8,
    hint: GridIndex,
}

impl Puzzle {
    pub fn new(grid: Grid<PuzzleCell>, swaps: u8, hint: GridIndex) -> Self {
        Self { grid, swaps, hint }
    }

    pub fn swap_limit(&self) -> u8 {
        self.swaps
    }

    pub fn start(&self) -> Grid<PuzzleCell> {
        self.grid.clone()
    }

    pub const fn hint(&self) -> GridIndex {
        self.hint
    }
}

pub fn fallback_puzzle() -> Puzzle {
    let mut grid = Grid::with_size(GridSize {
        width: 2,
        height: 1,
    });
    grid.insert(
        GridIndex { x: 0, y: 0 },
        PuzzleCell::Source {
            connections: DirectionSet::from_iter([Direction::W].into_iter()),
            source: Color::SWAP,
        },
    )
    .unwrap();
    grid.insert(
        GridIndex { x: 1, y: 0 },
        PuzzleCell::Normal {
            connections: DirectionSet::from_iter([Direction::E].into_iter()),
        },
    )
    .unwrap();
    Puzzle {
        grid,
        swaps: 1,
        hint: GridIndex { x: 1, y: 0 },
    }
}
