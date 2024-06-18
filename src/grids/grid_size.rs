use super::grid_index::GridIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct GridSize {
    pub width: usize,
    pub height: usize,
}

impl GridSize {
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub const fn contains(&self, index: GridIndex) -> bool {
        index.x < self.width && index.y < self.height
    }
}

pub struct GridSizeIter {
    size: GridSize,
    at: Option<GridIndex>,
}

impl Iterator for GridSizeIter {
    type Item = GridIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.at.take().map(|return_value| {
            self.at = if return_value.x + 1 < self.size.width {
                Some(GridIndex {
                    x: return_value.x + 1,
                    y: return_value.y,
                })
            } else if return_value.y + 1 < self.size.height {
                Some(GridIndex {
                    x: 0,
                    y: return_value.y + 1,
                })
            } else {
                None
            };
            return_value
        })
    }
}

impl IntoIterator for GridSize {
    type Item = GridIndex;

    type IntoIter = GridSizeIter;

    fn into_iter(self) -> Self::IntoIter {
        GridSizeIter {
            size: self,
            at: if self.width > 0 && self.height > 0 {
                Some(GridIndex { x: 0, y: 0 })
            } else {
                None
            },
        }
    }
}
