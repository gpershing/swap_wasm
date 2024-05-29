use std::iter;

use super::{grid_index::GridIndex, grid_size::GridSize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Grid<T> {
    data: Box<[Option<T>]>,
    size: GridSize
}

#[derive(Debug, Clone, Copy)]
pub struct IndexOutOfSize;
impl std::fmt::Display for IndexOutOfSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("index not included in size")
    }
}
impl std::error::Error for IndexOutOfSize {}

const fn get_linear_index(size: GridSize, grid_index: GridIndex) -> Option<usize> {
    if size.contains(grid_index) {
        Some(grid_index.x + grid_index.y * size.width)
    }
    else {
        None
    }
}

const unsafe fn get_linear_index_unchecked(size: GridSize, grid_index: GridIndex) -> usize {
    grid_index.x + grid_index.y * size.width
}

const fn get_index_from_linear_index(size: GridSize, linear_index: usize) -> GridIndex {
    GridIndex {
        x: linear_index % size.width,
        y: linear_index / size.width
    }
}

impl<T> Grid<T> {
    pub const fn with_size(size: GridSize) -> Self {
        let data = iter::repeat_with(|| None).take(size.width * size.height)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self { data, size }
    }

    pub const fn size(&self) -> GridSize {
        self.size
    }

    pub const fn width(&self) -> usize {
        self.size.width
    }

    pub const fn height(&self) -> usize {
        self.size.height
    }

    pub fn insert(&mut self, grid_index: GridIndex, value: T) -> Result<Option<T>, IndexOutOfSize> {
        let li = get_linear_index(self.size, grid_index).ok_or(IndexOutOfSize)?;
        let previous = self.data[li].take();
        self.data[li] = Some(value);
        Ok(previous)
    }

    pub fn remove(&mut self, grid_index: GridIndex) -> Result<Option<T>, IndexOutOfSize> {
        let li = get_linear_index(self.size, grid_index).ok_or(IndexOutOfSize)?;
        Ok(self.data[li].take())
    }

    pub const fn get(&self, grid_index: GridIndex) -> Option<&T> {
        get_linear_index(self.size, grid_index)
            .and_then(|li| self.data[li].as_ref())
    }

    pub const unsafe fn get_unchecked(&self, grid_index: GridIndex) -> Option<&T> {
        self.data.get_unchecked(get_linear_index_unchecked(self.size, grid_index))
            .as_ref()
    }

    pub fn get_mut(&mut self, grid_index: GridIndex) -> Option<&mut T> {
        get_linear_index(self.size, grid_index)
            .and_then(|li| self.data[li].as_mut())
    }

    pub unsafe fn get_unchecked_mut(&mut self, grid_index: GridIndex) -> Option<&mut T> {
        self.data.get_unchecked_mut(get_linear_index_unchecked(self.size, grid_index))
            .as_mut()
    }
    
    pub fn swap(&mut self, a: GridIndex, b: GridIndex) {
        self.data.swap(
            get_linear_index(self.size, a).unwrap(), 
            get_linear_index(self.size, b).unwrap())
    }

    pub fn indicies(&self) -> impl Iterator<Item = GridIndex> + '_ {
        (0..self.data.len())
            .filter(|li| self.data[*li].is_some())
            .map(|li| get_index_from_linear_index(self.size, li))
    }

    pub fn iter(&self) -> impl Iterator<Item = (GridIndex, &T)> + '_ {
        self.data.iter().enumerate()
            .filter_map(|(li, data)| data.as_ref().map(|d| (get_index_from_linear_index(self.size, li), d)))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (GridIndex, &mut T)> + '_ {
        self.data.iter_mut().enumerate()
            .filter_map(|(li, data)| data.as_mut().map(|d| (get_index_from_linear_index(self.size, li), d)))
    }
}