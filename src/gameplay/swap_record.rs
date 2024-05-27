use crate::grid_math::{Pos2, Rotation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwapRecord {
    pub(crate) a: Pos2,
    pub(crate) b: Pos2,
    pub(crate) a_rotation: Rotation,
    pub(crate) b_rotation: Rotation
}

impl SwapRecord {
    pub fn new(a: Pos2, b: Pos2, a_rotation: Rotation, b_rotation: Rotation) -> Self {
        Self { a, b, a_rotation, b_rotation }
    }
}