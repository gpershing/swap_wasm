use crate::grid_math::{DirSet, Rotation};

use super::{Color, ColorSet};

#[derive(Debug, Clone, Copy)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub struct Cell {
    id: usize,
    connections: DirSet,
    source: Option<Color>,
    fill: ColorSet
}

impl Cell {
    pub const fn new(id: usize, connections: DirSet) -> Self {
        Self {
            id,
            connections,
            source: None,
            fill: ColorSet::empty()
        }
    }

    pub const fn new_source(id: usize, connections: DirSet, source: Color) -> Self {
        Self {
            id,
            connections,
            source: Some(source),
            fill: ColorSet::singleton(source)
        }
    }

    pub const fn id(&self) -> usize {
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
}