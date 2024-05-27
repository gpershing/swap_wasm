use std::ops::Add;

use super::{Dir, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Pos2 {
    pub x: i8,
    pub y: i8
}

impl Pos2 {
    pub const ZERO: Pos2 = Self::new(0, 0);

    #[inline(always)]
    pub const fn new(x: i8, y: i8) -> Self {
        Self {
            x, y
        }
    }

    pub fn min(self, other: Pos2) -> Pos2 {
        Pos2 { x: self.x.min(other.x), y: self.y.min(other.y) }
    }

    pub fn max(self, other: Pos2) -> Pos2 {
        Pos2 { x: self.x.max(other.x), y: self.y.max(other.y) }
    }

    pub const fn to_posf(self) -> egui::Pos2 {
        egui::Pos2 { x: self.x as f32, y: self.y as f32 }
    }

    pub fn neighbors(self) -> impl Iterator<Item = Pos2> {
        Dir::ALL.iter().map(move |dir| self + dir.to_vec())
    }

    pub fn neighbors_using(self, dirs: impl Iterator<Item = Dir>) -> impl Iterator<Item = Pos2> {
        dirs.map(move |dir| self + dir.to_vec())
    }
}

impl From<(i8, i8)> for Pos2 {
    fn from(value: (i8, i8)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

impl From<&(i8, i8)> for Pos2 {
    fn from(value: &(i8, i8)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

impl From<[i8; 2]> for Pos2 {
    fn from(value: [i8; 2]) -> Self {
        Self { x: value[0], y: value[1] }
    }
}

impl From<&[i8; 2]> for Pos2 {
    fn from(value: &[i8; 2]) -> Self {
        Self { x: value[0], y: value[1] }
    }
}

impl Add<Vec2> for Pos2 {
    type Output = Pos2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Pos2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Add<Vec2> for &Pos2 {
    type Output = Pos2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Pos2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}