use std::ops::Range;

use super::{pos2::Pos2, vec2::Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Rect {
    origin: Pos2,
    size: Vec2
}

impl Rect {
    #[inline(always)]
    pub const fn new(origin: Pos2, size: Vec2) -> Self {
        Self {
            origin, size
        }
    }

    pub const fn from_extrema(min: Pos2, max: Pos2) -> Self {
        Self { origin: min, size: Vec2::new(max.x - min.x + 1, max.y - min.y + 1) }
    }

    pub const fn min_x(&self) -> i8 {
        self.origin.x
    }

    pub const fn min_y(&self) -> i8 {
        self.origin.y
    }

    pub const fn min(&self) -> Pos2 {
        Pos2 { x: self.min_x(), y: self.min_y() }
    }

    pub const fn max_x(&self) -> i8 {
        self.origin.x + self.size.x - 1
    }

    pub const fn max_y(&self) -> i8 {
        self.origin.y + self.size.y - 1
    }

    pub const fn max(&self) -> Pos2 {
        Pos2 { x: self.max_x(), y: self.max_y() }
    }

    pub const fn end_x(&self) -> i8 {
        self.origin.x + self.size.x
    }

    pub const fn end_y(&self) -> i8 {
        self.origin.y + self.size.y
    }

    pub const fn contains(&self, point: Pos2) -> bool {
        self.min_x() <= point.x && point.x <= self.max_x() &&
        self.min_y() <= point.y && point.y <= self.max_y()
    }

    pub fn including(&self, point: Pos2) -> Self {
        Self::from_extrema(self.min().min(point), self.max().max(point))
    }
}