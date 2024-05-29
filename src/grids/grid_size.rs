use super::grid_index::GridIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GridSize {
    pub width: usize,
    pub height: usize
}

impl GridSize {
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub const fn contains(&self, index: GridIndex) -> bool {
        index.x < self.width && index.y < self.height
    }
}