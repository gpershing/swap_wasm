use super::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct GridIndex {
    pub x: usize,
    pub y: usize,
}

impl GridIndex {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn moved_in(self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::E => Some(GridIndex {
                x: self.x + 1,
                y: self.y,
            }),
            Direction::N => self.y.checked_sub(1).map(|y| GridIndex { x: self.x, y }),
            Direction::W => self.x.checked_sub(1).map(|x| GridIndex { x, y: self.y }),
            Direction::S => Some(GridIndex {
                x: self.x,
                y: self.y + 1,
            }),
        }
    }
}
