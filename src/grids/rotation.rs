#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Rotation {
    None,
    CounterClockwise,
    Half,
    Clockwise
}

impl Rotation {
    pub const fn inverse(self) -> Rotation {
        match self {
            Rotation::None => Rotation::None,
            Rotation::CounterClockwise => Rotation::Clockwise,
            Rotation::Half => Rotation::Half,
            Rotation::Clockwise => Rotation::CounterClockwise,
        }
    }
}