use std::ops::{Index, IndexMut};

use super::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DirectionMap<T> {
    data: [T; 4]
}

pub type DirectionSet = DirectionMap<bool>;

const fn idx(direction: Direction) -> usize {
    match direction {
        Direction::E => 0,
        Direction::N => 1,
        Direction::W => 2,
        Direction::S => 3,
    }
}

const fn dir_for_idx(idx: usize) -> Direction {
    match idx {
        0 => Direction::E,
        1 => Direction::N,
        2 => Direction::W,
        3 => Direction::S,
        _ => panic!()
    }
}

impl<T> DirectionMap<T> {
    pub fn new_with_initial(f: impl Fn() -> T) -> Self {
        let data = [f(), f(), f(), f()];
        Self { data }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Direction, &T)> {
        Direction::ALL.iter().map(|dir| (*dir, &self[dir]))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Direction, &mut T)> {
        self.data.iter_mut().enumerate()
            .map(|(idx, el)| (dir_for_idx(idx), el))
    }

    pub fn map<Other>(&self, f: impl Fn(&T) -> Other) -> DirectionMap<Other> {
        DirectionMap {
            data: self.data.map(f)
        }
    }
}

impl<T> DirectionMap<T> where T : Copy {
    pub fn new_with_repeat(value: T) -> Self {
        let data = [value; 4];
        Self { data }
    }
}

impl<T> Index<Direction> for DirectionMap<T> {
    type Output = T;

    fn index(&self, index: Direction) -> &Self::Output {
        &self.data[idx(index)]
    }
}

impl<T> Index<&Direction> for DirectionMap<T> {
    type Output = T;

    fn index(&self, index: &Direction) -> &Self::Output {
        &self.data[idx(*index)]
    }
}

impl<T> IndexMut<Direction> for DirectionMap<T> {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        &mut self.data[idx(index)]
    }
}

impl<T> IndexMut<&Direction> for DirectionMap<T> {
    fn index_mut(&mut self, index: &Direction) -> &mut Self::Output {
        &mut self.data[idx(*index)]
    }
}

impl DirectionSet {
    pub fn contains(&self, direction: Direction) -> bool {
        self[direction]
    }

    pub fn insert(&mut self, direction: Direction) -> bool {
        let rv = self[direction];
        self[direction] = true;
        rv
    }

    pub fn remove(&mut self, direction: Direction) -> bool {
        let rv = self[direction];
        self[direction] = false;
        rv
    }

    pub fn iter_set(&self) -> impl Iterator<Item = Direction> + '_ {
        Direction::ALL.into_iter().filter(|d| self[d])
    }
}