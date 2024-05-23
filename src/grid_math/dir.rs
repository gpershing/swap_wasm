use super::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    E,
    N,
    W,
    S
}

impl Dir {
    pub const ALL: [Dir; 4] = [Dir::E, Dir::N, Dir::W, Dir::S];

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