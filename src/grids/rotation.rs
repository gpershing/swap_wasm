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

    pub const fn rotated(self, other: Rotation) -> Rotation {
        match (self, other) {
            (Rotation::None, x) => x,
            (x, Rotation::None) => x,
            (Rotation::Half, x) => x.inverse(),
            (x, Rotation::Half) => x.inverse(),
            (Rotation::CounterClockwise, Rotation::CounterClockwise) => Rotation::Half,
            (Rotation::CounterClockwise, Rotation::Clockwise) => Rotation::None,
            (Rotation::Clockwise, Rotation::CounterClockwise) => Rotation::None,
            (Rotation::Clockwise, Rotation::Clockwise) => Rotation::Half,
        }
    }
}