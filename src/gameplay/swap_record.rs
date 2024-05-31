use crate::grids::{GridIndex, Rotation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwapRecord {
    pub(crate) a: GridIndex,
    pub(crate) b: GridIndex,
    pub(crate) a_rotation: Rotation,
    pub(crate) b_rotation: Rotation
}

impl SwapRecord {
    pub fn new(a: GridIndex, b: GridIndex, a_rotation: Rotation, b_rotation: Rotation) -> Self {
        Self { a, b, a_rotation, b_rotation }
    }
}