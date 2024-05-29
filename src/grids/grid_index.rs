#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GridIndex {
    pub x: usize,
    pub y: usize
}

impl GridIndex {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}