use std::iter;

use super::{grid_index::GridIndex, GridSize, Direction};

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Grid<T> {
    data: Box<[Option<T>]>,
    size: GridSize,
    filled: usize,
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
    pub fn with_size(size: GridSize) -> Self {
        let data = iter::repeat_with(|| None).take(size.width * size.height)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self { data, filled: 0, size }
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

    pub const fn len(&self) -> usize {
        self.filled
    }

    pub fn insert(&mut self, grid_index: GridIndex, value: T) -> Result<Option<T>, IndexOutOfSize> {
        let li = get_linear_index(self.size, grid_index).ok_or(IndexOutOfSize)?;
        let previous = self.data[li].take();
        self.data[li] = Some(value);
        if previous.is_none() { self.filled += 1 };
        Ok(previous)
    }

    pub fn remove(&mut self, grid_index: GridIndex) -> Result<Option<T>, IndexOutOfSize> {
        let li = get_linear_index(self.size, grid_index).ok_or(IndexOutOfSize)?;
        let previous = self.data[li].take();
        if previous.is_some() { self.filled -= 1 };
        Ok(previous)
    }

    pub fn contains(&self, grid_index: GridIndex) -> bool {
        self.get(grid_index).is_some()
    }

    pub fn get(&self, grid_index: GridIndex) -> Option<&T> {
        get_linear_index(self.size, grid_index)
            .and_then(|li| self.data[li].as_ref())
    }

    pub unsafe fn get_unchecked(&self, grid_index: GridIndex) -> Option<&T> {
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

impl<T> Grid<T> {
    pub fn get_neighbor(&self, index: GridIndex, direction: Direction) -> Option<(GridIndex, &T)> {
        let index = index.moved_in(direction)?;
        self.get(index).map(|item| (index, item))
    }

    pub fn get_neighbor_mut(&mut self, index: GridIndex, direction: Direction) -> Option<(GridIndex, &mut T)> {
        let index = index.moved_in(direction)?;
        self.get_mut(index).map(|item| (index, item))
    }

    pub fn iter_neighbors_for(&self, index: GridIndex, directions: impl Iterator<Item = Direction>) -> impl Iterator<Item = (GridIndex, Direction, &T)> {
        directions.filter_map(move |dir| self.get_neighbor(index, dir).map(|(idx, item)| (idx, dir, item)))
    }

    pub fn iter_neighbors(&self, index: GridIndex) -> impl Iterator<Item = (GridIndex, Direction, &T)> {
        self.iter_neighbors_for(index, Direction::ALL.into_iter())
    }

    pub fn neighbors_mut_for(&mut self, index: GridIndex, directions: impl Iterator<Item = Direction>) -> impl Iterator<Item = (GridIndex, Direction, &mut T)> {
        let mut linear_indices: Vec<_> = directions
            .filter_map(|dir| index
                .moved_in(dir)
                .and_then(|idx| get_linear_index(self.size, idx))
                .map(|idx| (idx, index, dir)))
            .collect();
        linear_indices.sort_by(|a, b| a.0.cmp(&b.0));

        let mut refs = Vec::new();
        let mut all_refs = self.data.iter_mut();
        let mut last_index = 0;
        for (index, grid_index, dir) in linear_indices {
            if let Some(Some(rf)) = all_refs.nth(index - last_index) {
                refs.push((grid_index, dir, rf));
            }
            last_index = index;
        }
        refs.into_iter()
    }

    pub fn neighbors_mut(&mut self, index: GridIndex) -> impl Iterator<Item = (GridIndex, Direction, &mut T)> {
        self.neighbors_mut_for(index, Direction::ALL.into_iter())
    }
}

pub struct GridIter<T> {
    grid: Grid<T>,
    index_iter: <GridSize as IntoIterator>::IntoIter
}

impl<T> Iterator for GridIter<T> {
    type Item = (GridIndex, T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(index) = self.index_iter.next() {
                unsafe {
                    match self.grid.data[get_linear_index_unchecked(self.grid.size, index)].take() {
                        Some(item) => break Some((index, item)),
                        None => (),
                    }
                }
            }
            else {
                break None
            }
        }
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = (GridIndex, T);

    type IntoIter = GridIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let size = self.size;
        GridIter {
            grid: self,
            index_iter: size.into_iter()
        }
    }
}