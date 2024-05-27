#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Rotation {
    None,
    CCW,
    Half,
    CW
}

impl Rotation {
    pub const fn inverse(self) -> Rotation {
        match self {
            Rotation::None => Rotation::None,
            Rotation::CCW => Rotation::CW,
            Rotation::Half => Rotation::Half,
            Rotation::CW => Rotation::CCW,
        }
    }
}