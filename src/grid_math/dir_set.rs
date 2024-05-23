use std::ops::{Index, IndexMut};

use super::Dir;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DirSet {
    e: bool,
    n: bool,
    w: bool,
    s: bool
}

impl DirSet {
    pub const fn new() -> Self {
        Self { e: false, n: false, w: false, s: false }
    }

    pub const fn ordered(e: bool, n: bool, w: bool, s: bool) -> Self {
        Self { e, n, w, s }
    }

    pub const fn len(&self) -> usize {
        let mut len = 0;
        if self.e { len += 1 };
        if self.n { len += 1 };
        if self.w { len += 1 };
        if self.s { len += 1 }
        len
    }

    pub const fn inverse(self) -> Self {
        Self { e: !self.e, n: !self.n, w: !self.w, s: !self.s }
    }

    pub fn iter(&self) -> impl Iterator<Item = Dir> + '_ {
        Dir::ALL.into_iter().filter(|d| self[*d])
    }
}

impl Index<Dir> for DirSet {
    type Output = bool;

    fn index(&self, index: Dir) -> &Self::Output {
        match index {
            Dir::E => &self.e,
            Dir::N => &self.n,
            Dir::W => &self.w,
            Dir::S => &self.s,
        }
    }
}

impl IndexMut<Dir> for DirSet {
    fn index_mut(&mut self, index: Dir) -> &mut Self::Output {
        match index {
            Dir::E => &mut self.e,
            Dir::N => &mut self.n,
            Dir::W => &mut self.w,
            Dir::S => &mut self.s,
        }
    }
}