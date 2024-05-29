use crate::grids::{DirectionSet, Rotation};

use super::{cell_id::CellId, puzzle::{LayerConnection, PuzzleCell}, Color, ColorSet};

#[derive(Debug, Clone, Copy)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub struct  CellLayer {
    connections: DirectionSet,
    fill: ColorSet
}

#[derive(Debug, Clone, Copy)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub enum CellData {
    Normal {
        layer: CellLayer,
        source: Option<Color>
    },
    Intersection {
        layers: [CellLayer; 2]
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
            PuzzleCell::Normal { connections } => CellData::Normal { layer: CellLayer { connections, fill: ColorSet::empty() }, source: None },
            PuzzleCell::Source { connections, source } => CellData::Normal { layer: CellLayer { connections, fill: ColorSet::singleton(source) }, source: Some(source) },
            PuzzleCell::Intersection { connections } => CellData::Intersection { layers: [
                CellLayer { connections: connections.map(|c| c == LayerConnection::Layer0), fill: ColorSet::empty() },
                CellLayer { connections: connections.map(|c| c == LayerConnection::Layer1), fill: ColorSet::empty() }
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

    pub const fn get_layer_count(&self) -> usize {
        match &self.data {
            CellData::Normal { layer, source } => 1,
            CellData::Intersection { layers } => 2,
        }
    }

    pub const fn get_layer(&self, layer_idx: usize) -> Option<&CellLayer> {
        match &self.data {
            CellData::Normal { layer, source } => match layer_idx {
                0 => Some(layer),
                _ => None
            },
            CellData::Intersection { layers } => match layer_idx {
                0 => Some(&layers[0]),
                1 => Some(&layers[1]),
                _ => None
            }
        }
    }

    pub const fn get_layer_mut(&mut self, layer_idx: usize) -> Option<&mut CellLayer> {
        match &mut self.data {
            CellData::Normal { layer, source } => match layer_idx {
                0 => Some(layer),
                _ => None
            },
            CellData::Intersection { layers } => match layer_idx {
                0 => Some(&mut layers[0]),
                1 => Some(&mut layers[1]),
                _ => None
            }
        }
    }

    pub const fn source(&self) -> Option<Color> {
        match self.data {
            CellData::Normal { layer, source } => source,
            CellData::Intersection { layers } => None,
        }
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