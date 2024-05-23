#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Vec2 {
    pub x: i8,
    pub y: i8
}

impl Vec2 {
    pub const ZERO: Vec2 = Self::new(0, 0);

    #[inline(always)]
    pub const fn new(x: i8, y: i8) -> Self {
        Self {
            x, y
        }
    }
}

impl From<(i8, i8)> for Vec2 {
    fn from(value: (i8, i8)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

impl From<&(i8, i8)> for Vec2 {
    fn from(value: &(i8, i8)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

impl From<[i8; 2]> for Vec2 {
    fn from(value: [i8; 2]) -> Self {
        Self { x: value[0], y: value[1] }
    }
}

impl From<&[i8; 2]> for Vec2 {
    fn from(value: &[i8; 2]) -> Self {
        Self { x: value[0], y: value[1] }
    }
}