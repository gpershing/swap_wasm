use crate::grids::{Direction, DirectionMap, DirectionSet, Rotation};

use super::{
    puzzle::{LayerConnection, PuzzleCell},
    Color, ColorSet,
};

#[derive(Debug, Clone, Copy, serde::Serialize, serde:: Deserialize)]
pub struct CellLayer {
    pub connections: DirectionSet,
    pub fill: ColorSet,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde:: Deserialize)]
pub enum CellData {
    Normal {
        layer: CellLayer,
        source: Option<Color>,
    },
    Intersection {
        layers: [CellLayer; 2],
    },
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde:: Deserialize)]
pub struct Cell {
    data: CellData,
}

impl Cell {
    pub fn new(puzzle_cell: PuzzleCell) -> Self {
        let data = match puzzle_cell {
            PuzzleCell::Normal { connections } => CellData::Normal {
                layer: CellLayer {
                    connections,
                    fill: ColorSet::empty(),
                },
                source: None,
            },
            PuzzleCell::Source {
                connections,
                source,
            } => CellData::Normal {
                layer: CellLayer {
                    connections,
                    fill: ColorSet::singleton(source),
                },
                source: Some(source),
            },
            PuzzleCell::Intersection { connections } => CellData::Intersection {
                layers: [
                    CellLayer {
                        connections: connections.map(|c| c == LayerConnection::Layer0),
                        fill: ColorSet::empty(),
                    },
                    CellLayer {
                        connections: connections.map(|c| c == LayerConnection::Layer1),
                        fill: ColorSet::empty(),
                    },
                ],
            },
        };
        Self { data }
    }

    pub fn to_puzzle_cell(self) -> PuzzleCell {
        match self.data {
            CellData::Normal { layer, source } => match source {
                Some(source) => PuzzleCell::Source {
                    connections: layer.connections,
                    source,
                },
                None => PuzzleCell::Normal {
                    connections: layer.connections,
                },
            },
            CellData::Intersection { layers } => {
                let mut connections = DirectionMap::default();
                for dir in layers[0].connections.iter_set() {
                    connections[dir] = LayerConnection::Layer0;
                }
                for dir in layers[1].connections.iter_set() {
                    connections[dir] = LayerConnection::Layer1;
                }
                PuzzleCell::Intersection { connections }
            }
        }
    }

    pub fn total_connections(&self) -> usize {
        match self.data {
            CellData::Normal { layer, source: _ } => layer.connections.len(),
            CellData::Intersection { layers } => {
                layers[0].connections.len() + layers[1].connections.len()
            }
        }
    }

    pub const fn get_layer_count(&self) -> usize {
        match &self.data {
            CellData::Normal {
                layer: _layer,
                source: _source,
            } => 1,
            CellData::Intersection { layers: _layers } => 2,
        }
    }

    pub const fn get_layer(&self, layer_idx: usize) -> Option<&CellLayer> {
        match &self.data {
            CellData::Normal {
                layer,
                source: _source,
            } => match layer_idx {
                0 => Some(layer),
                _ => None,
            },
            CellData::Intersection { layers } => match layer_idx {
                0 => Some(&layers[0]),
                1 => Some(&layers[1]),
                _ => None,
            },
        }
    }

    pub fn get_layer_mut(&mut self, layer_idx: usize) -> Option<&mut CellLayer> {
        match &mut self.data {
            CellData::Normal {
                layer,
                source: _source,
            } => match layer_idx {
                0 => Some(layer),
                _ => None,
            },
            CellData::Intersection { layers } => match layer_idx {
                0 => Some(&mut layers[0]),
                1 => Some(&mut layers[1]),
                _ => None,
            },
        }
    }

    pub fn iter_layers(&self) -> impl Iterator<Item = &CellLayer> {
        match &self.data {
            CellData::Normal {
                layer,
                source: _source,
            } => vec![layer],
            CellData::Intersection { layers } => layers.iter().collect(),
        }
        .into_iter()
    }

    pub fn get_layer_for_direction(&self, direction: Direction) -> Option<(usize, &CellLayer)> {
        match &self.data {
            CellData::Normal {
                layer,
                source: _source,
            } => {
                if layer.connections.contains(direction) {
                    Some((0, layer))
                } else {
                    None
                }
            }
            CellData::Intersection { layers } => {
                if layers[0].connections.contains(direction) {
                    Some((0, &layers[0]))
                } else if layers[1].connections.contains(direction) {
                    Some((1, &layers[1]))
                } else {
                    None
                }
            }
        }
    }

    pub const fn source(&self) -> Option<Color> {
        match self.data {
            CellData::Normal {
                layer: _layer,
                source,
            } => source,
            CellData::Intersection { layers: _layers } => None,
        }
    }

    pub(crate) fn rotate(&mut self, rotation: Rotation) {
        match &mut self.data {
            CellData::Normal {
                layer,
                source: _source,
            } => {
                layer.connections = layer.connections.rotated(rotation);
            }
            CellData::Intersection { layers } => {
                layers[0].connections = layers[0].connections.rotated(rotation);
                layers[1].connections = layers[1].connections.rotated(rotation);
            }
        }
    }

    pub fn rotation_for_fill(&self) -> Rotation {
        let ccw = self.has_color_in_any_layer(Color::CCW);
        let cw = self.has_color_in_any_layer(Color::CW);
        match (ccw, cw) {
            (true, true) => Rotation::None,
            (true, false) => Rotation::CounterClockwise,
            (false, true) => Rotation::Clockwise,
            (false, false) => Rotation::None,
        }
    }

    pub(crate) fn rotate_by_fill(&mut self) -> Rotation {
        let rotation = self.rotation_for_fill();
        self.rotate(rotation);
        rotation
    }

    pub fn clear_fill(&mut self) {
        match &mut self.data {
            CellData::Normal { layer, source: _ } => layer.fill = ColorSet::empty(),
            CellData::Intersection { layers } => {
                layers[0].fill = ColorSet::empty();
                layers[1].fill = ColorSet::empty();
            }
        }
    }

    pub fn has_color_in_any_layer(&self, color: Color) -> bool {
        match self.data {
            CellData::Normal {
                layer,
                source: _source,
            } => layer.fill.contains(color),
            CellData::Intersection { layers } => {
                layers[0].fill.contains(color) || layers[1].fill.contains(color)
            }
        }
    }

    pub fn can_swap(first_cell: &Cell, second_cell: &Cell) -> bool {
        (first_cell.has_color_in_any_layer(Color::SWAP)
            || second_cell.has_color_in_any_layer(Color::SWAP))
            && !first_cell.has_color_in_any_layer(Color::STOP)
            && !second_cell.has_color_in_any_layer(Color::STOP)
    }
}
