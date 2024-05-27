use super::{Rotation, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    E,
    N,
    W,
    S
}

impl Dir {
    pub const ALL: [Dir; 4] = [Dir::E, Dir::N, Dir::W, Dir::S];

    pub const fn inverse(self) -> Dir {
        match self {
            Dir::E => Dir::W,
            Dir::N => Dir::S,
            Dir::W => Dir::E,
            Dir::S => Dir::N,
        }
    }

    pub const fn rotated(self, rotation: Rotation) -> Dir {
        match (self, rotation) {
            (Dir::E, Rotation::None) => Dir::E,
            (Dir::E, Rotation::CCW) => Dir::N,
            (Dir::E, Rotation::Half) => Dir::W,
            (Dir::E, Rotation::CW) => Dir::S,
            (Dir::N, Rotation::None) => Dir::N,
            (Dir::N, Rotation::CCW) => Dir::W,
            (Dir::N, Rotation::Half) => Dir::S,
            (Dir::N, Rotation::CW) => Dir::E,
            (Dir::W, Rotation::None) => Dir::W,
            (Dir::W, Rotation::CCW) => Dir::S,
            (Dir::W, Rotation::Half) => Dir::E,
            (Dir::W, Rotation::CW) => Dir::N,
            (Dir::S, Rotation::None) => Dir::S,
            (Dir::S, Rotation::CCW) => Dir::E,
            (Dir::S, Rotation::Half) => Dir::N,
            (Dir::S, Rotation::CW) => Dir::W,
        }
    }

    pub const fn to_vec(self) -> Vec2 {
        match self {
            Self::E => Vec2 { x: 1, y: 0 },
            Self::N => Vec2 { x: 0, y: -1 },
            Self::W => Vec2 { x: -1, y: 0 },
            Self::S => Vec2 { x: 0, y: 1 }
        }
    }

    pub const fn to_vecf(self) -> egui::Vec2 {
        match self {
            Self::E => egui::Vec2 { x: 1.0, y: 0.0 },
            Self::N => egui::Vec2 { x: 0.0, y: -1.0 },
            Self::W => egui::Vec2 { x: -1.0, y: 0.0 },
            Self::S => egui::Vec2 { x: 0.0, y: 1.0 }
        }
    }
}