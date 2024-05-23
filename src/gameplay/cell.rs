use crate::grid_math::DirSet;

#[derive(serde::Serialize, serde:: Deserialize)]
pub struct Cell {
    id: usize,
    connections: DirSet
}

impl Cell {
    pub const fn new(id: usize, connections: DirSet) -> Self {
        Self {
            id,
            connections
        }
    }

    pub const fn id(&self) -> usize {
        self.id
    }

    pub const fn connections(&self) -> DirSet {
        self.connections
    }
}