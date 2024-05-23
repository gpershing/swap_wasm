use std::path::Iter;

use egui::ahash::{HashMap, HashMapExt};

use super::{dir::Dir, Pos2, Rect, Vec2};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Grid<T> {
    data: HashMap<Pos2, T>,
    bounds: Rect
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self { data: HashMap::new(), bounds: Rect::default() }
    }

    pub fn get(&self, pos: Pos2) -> Option<&T> {
        self.data.get(&pos)
    }

    pub fn get_mut(&mut self, pos: Pos2) -> Option<&mut T> {
        self.data.get_mut(&pos)
    }
    
    pub fn insert(&mut self, pos: Pos2, value: T) -> Option<T> {
        match self.data.insert(pos, value) {
            Some(prev) => Some(prev),
            None => {
                if self.data.len() == 1 {
                    self.bounds = Rect::new(pos, Vec2::new(1, 1))
                }
                else {
                    self.bounds = self.bounds.including(pos)
                };
                None
            },
        }
    }

    pub fn remove(&mut self, pos: Pos2) -> Option<T> {
        self.data.remove(&pos)
    }

    pub fn iter(&self) -> impl Iterator<Item=(&Pos2, &T)> {
        self.data.iter()
    }

    pub fn recompute_bounds(&mut self) {
        let mut keys = self.data.keys().peekable();
        self.bounds = match keys.peek() {
            Some(pos) => Rect::new(**pos, Vec2::new(1, 1)),
            None => Rect::default(),
        };
        for pos in keys {
            self.bounds = self.bounds.including(*pos)
        }
    }

    pub fn neighbors(&self, pos: Pos2) -> impl Iterator<Item = &T> {
        Dir::ALL.iter().map(move |dir| self.get(pos + dir.to_vec()))
            .filter_map(std::convert::identity)
    }

    pub fn neighbors_by_dir(&self, pos: Pos2) -> impl Iterator<Item = (Dir, &T)> {
        Dir::ALL.iter().map(move |dir| (dir, self.get(pos + dir.to_vec())))
            .filter_map(|(d, p)| p.map(|pos| (*d, pos)))
    }

    pub fn neighbors_by_point(&self, pos: Pos2) -> impl Iterator<Item = (Pos2, &T)> {
        Dir::ALL.iter().map(move |dir| (pos + dir.to_vec(), self.get(pos + dir.to_vec())))
            .filter_map(|(d, p)| p.map(|pos| (d, pos)))
    }
}