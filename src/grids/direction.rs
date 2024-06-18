use egui::Vec2;

use super::Rotation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    E,
    N,
    W,
    S,
}

impl Direction {
    pub const ALL: [Direction; 4] = [Direction::E, Direction::N, Direction::W, Direction::S];

    pub const fn inverse(self) -> Direction {
        match self {
            Direction::E => Direction::W,
            Direction::N => Direction::S,
            Direction::W => Direction::E,
            Direction::S => Direction::N,
        }
    }

    pub const fn rotated(self, rotation: Rotation) -> Direction {
        match (self, rotation) {
            (Direction::E, Rotation::None) => Direction::E,
            (Direction::E, Rotation::CounterClockwise) => Direction::N,
            (Direction::E, Rotation::Half) => Direction::W,
            (Direction::E, Rotation::Clockwise) => Direction::S,
            (Direction::N, Rotation::None) => Direction::N,
            (Direction::N, Rotation::CounterClockwise) => Direction::W,
            (Direction::N, Rotation::Half) => Direction::S,
            (Direction::N, Rotation::Clockwise) => Direction::E,
            (Direction::W, Rotation::None) => Direction::W,
            (Direction::W, Rotation::CounterClockwise) => Direction::S,
            (Direction::W, Rotation::Half) => Direction::E,
            (Direction::W, Rotation::Clockwise) => Direction::N,
            (Direction::S, Rotation::None) => Direction::S,
            (Direction::S, Rotation::CounterClockwise) => Direction::E,
            (Direction::S, Rotation::Half) => Direction::N,
            (Direction::S, Rotation::Clockwise) => Direction::W,
        }
    }

    pub const fn to_vec(self) -> Vec2 {
        match self {
            Direction::E => Vec2 { x: 1.0, y: 0.0 },
            Direction::N => Vec2 { x: 0.0, y: -1.0 },
            Direction::W => Vec2 { x: -1.0, y: 0.0 },
            Direction::S => Vec2 { x: 0.0, y: 1.0 },
        }
    }
}
