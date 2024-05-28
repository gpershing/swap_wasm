use crate::grid_math::{DirSet, Rotation};

use super::{cell_id::CellId, puzzle::PuzzleCell, Color, ColorSet};

#[derive(Debug, Clone, Copy)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub struct Cell {
    id: CellId,
    connections: DirSet,
    source: Option<Color>,
    fill: ColorSet
}

impl Cell {
    pub const fn new(id: CellId, puzzle_cell: PuzzleCell) -> Self {
        Self {
            id,
            connections: puzzle_cell.connections,
            source: puzzle_cell.source,
            fill: match puzzle_cell.source {
                Some(source) => ColorSet::singleton(source),
                None => ColorSet::empty(),
            }
        }
    }

    pub const fn id(&self) -> CellId {
        self.id
    }

    pub const fn connections(&self) -> DirSet {
        self.connections
    }

    pub(crate) fn rotate_connections(&mut self, rotation: Rotation) {
        self.connections = self.connections.rotated(rotation)
    }

    pub const fn source(&self) -> Option<Color> {
        self.source
    }

    pub const fn fill(&self) -> ColorSet {
        self.fill
    }

    pub fn set_min_fill(&mut self) {
        self.set_fill(match self.source {
            Some(source) => ColorSet::singleton(source),
            None => ColorSet::empty(),
        });
    }

    pub fn fill_mut(&mut self) -> &mut ColorSet {
        &mut self.fill
    }

    pub fn set_fill(&mut self, fill: ColorSet) {
        self.fill = fill;
    }

    pub fn can_swap(first_cell: &Cell, second_cell: &Cell) -> bool {
        (first_cell.fill().contains(Color::Purple) || second_cell.fill().contains(Color::Purple))
            && !first_cell.fill().contains(Color::Red)
            && !second_cell.fill().contains(Color::Red)
    }
}