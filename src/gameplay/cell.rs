use crate::grid_math::{DirSet, Rotation};

use super::{cell_id::CellId, puzzle::PuzzleCell, Color, ColorSet};

#[derive(Debug, Clone, Copy)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub struct  CellDataLayer {
    connections: DirSet,
    fill: ColorSet
}

#[derive(Debug, Clone, Copy)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub enum CellData {
    Normal {
        layer: CellDataLayer,
        source: Option<Color>
    },
    Intersection {
        layers: [CellDataLayer; 2]
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub struct Cell {
    id: CellId,
    data: CellData
}

impl Cell {
    pub const fn new(id: CellId, puzzle_cell: PuzzleCell) -> Self {
        let data = match puzzle_cell {
            PuzzleCell::Normal { connections } => CellData::Normal { layer: CellDataLayer { connections, fill: ColorSet::empty() }, source: None },
            PuzzleCell::Source { connections, source } => CellData::Normal { layer: CellDataLayer { connections, fill: ColorSet::singleton(source) }, source: Some(source) },
            PuzzleCell::Intersection { layers } => CellData::Intersection { layers: [
                CellDataLayer { connections: layers[0], fill: ColorSet::empty() },
                CellDataLayer { connections: layers[1], fill: ColorSet::empty() }
            ] },
        };
        Self {
            id,
            data
        }
    }

    pub const fn id(&self) -> CellId {
        self.id
    }

    pub(crate) fn rotate(&mut self, rotation: Rotation) {
        match &mut self.data {
            CellData::Normal { layer, source } => {
                layer.connections = layer.connections.rotated(rotation);
            },
            CellData::Intersection { layers } => {
                layers[0].connections = layers[0].connections.rotated(rotation);
                layers[1].connections = layers[1].connections.rotated(rotation);
            }
        }
    }

    pub const fn source(&self) -> Option<Color> {
        match self.data {
            CellData::Normal { layer, source } => source,
            CellData::Intersection { layers } => None,
        }
    }

    pub fn set_min_fill(&mut self) {
        match &mut self.data {
            CellData::Normal { layer, source } => {
                layer.fill = match source {
                    Some(source) => ColorSet::singleton(*source),
                    None => ColorSet::empty(),
                }
            },
            CellData::Intersection { layers } => {
                layers[0].fill = ColorSet::empty();
                layers[1].fill = ColorSet::empty();
            },
        }
    }

    fn has_color_in_any_layer(&self, color: Color) -> bool {
        match self.data {
            CellData::Normal { layer, source } => {
                layer.fill.contains(color)
            },
            CellData::Intersection { layers } => {
                layers[0].fill.contains(color) || layers[1].fill.contains(color)
            },
        }
    }

    pub fn can_swap(first_cell: &Cell, second_cell: &Cell) -> bool {
        (first_cell.has_color_in_any_layer(Color::SWAP) || second_cell.has_color_in_any_layer(Color::SWAP))
            && !first_cell.has_color_in_any_layer(Color::STOP)
            && !second_cell.has_color_in_any_layer(Color::STOP)
    }
}