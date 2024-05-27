use std::collections::VecDeque;

use egui::ahash::{HashSet, HashSetExt};

use crate::grid_math::{DirSet, Grid, Pos2, Rect, Rotation};

use super::{cell::Cell, Color, ColorSet, SwapRecord};

#[derive(serde::Serialize, serde:: Deserialize)]
pub struct Puzzle {
    grid: Grid<Cell>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub enum PuzzleSolveState {
    Solved,
    NotAllConnected,
    NotAllFilled,
    DoubleFilled,
    DuplicateColorSection
}

impl Puzzle {
    pub fn new() -> Self {
        let mut grid = Grid::new();
        grid.insert((0, 0).into(), Cell::new(0, DirSet::ordered(false, false, false, true)));
        grid.insert((0, 1).into(), Cell::new_source(1, DirSet::ordered(false, true, false, false), super::Color::Orange));
        grid.insert((1, 0).into(), Cell::new(2, DirSet::ordered(false, false, false, true)));
        grid.insert((1, 1).into(), Cell::new_source(3, DirSet::ordered(false, true, false, false), super::Color::Purple));
        let mut p = Self { grid };
        p.compute_fill();
        p
    }

    pub fn cells(&self) -> impl Iterator<Item = (&Pos2, &Cell)> {
        self.grid.iter()
    }
    
    pub fn get(&self, pos: Pos2) -> Option<&Cell> {
        self.grid.get(pos)
    }

    pub fn bounds(&self) -> Rect {
        self.grid.bounds()
    }
    
    pub fn entry_for_id(&self, id: usize) -> Option<(&Pos2, &Cell)> {
        self.grid.iter().find(|(_pos, cell)| cell.id() == id)
    }
    
    pub fn entry_mut_for_id(&mut self, id: usize) -> Option<(&Pos2, &mut Cell)> {
        self.grid.iter_mut().find(|(_pos, cell)| cell.id() == id)
    }
    
    fn rotation_for(color_set: ColorSet) -> Rotation {
        match (color_set.contains(Color::Orange), color_set.contains(Color::Yellow)) {
            (true, true) => Rotation::None,
            (true, false) => Rotation::CCW,
            (false, true) => Rotation::CW,
            (false, false) => Rotation::None,
        }
    }

    fn swap(&mut self, first_pos: Pos2, mut first_cell: Cell, second_pos: Pos2, mut second_cell: Cell) -> SwapRecord {
        let first_rotation = Self::rotation_for(first_cell.fill());
        let second_rotation = Self::rotation_for(second_cell.fill());
        first_cell.rotate_connections(first_rotation);
        second_cell.rotate_connections(second_rotation);
        self.grid.insert(first_pos, second_cell);
        self.grid.insert(second_pos, first_cell);
        self.compute_fill();
        SwapRecord::new(first_pos, second_pos, first_rotation, second_rotation)
    }

    fn undo_swap(&mut self, record: SwapRecord, mut first_cell: Cell, mut second_cell: Cell) {
        second_cell.rotate_connections(record.a_rotation.inverse());
        first_cell.rotate_connections(record.b_rotation.inverse());
        self.grid.insert(record.a, second_cell);
        self.grid.insert(record.b, first_cell);
        self.compute_fill();
    }

    fn can_swap(first_pos: Pos2, first_cell: Cell, second_pos: Pos2, second_cell: Cell) -> bool {
        first_pos != second_pos
        && (first_cell.fill().contains(Color::Purple) || second_cell.fill().contains(Color::Purple))
        && !first_cell.fill().contains(Color::Red)
        && !second_cell.fill().contains(Color::Red)
    }

    pub fn try_swap(&mut self, first: usize, second: usize) -> Option<SwapRecord> {
        let first = self.entry_for_id(first);
        let second = self.entry_for_id(second);
        if let Some((&first_pos, &first_cell)) = first {
            if let Some((&second_pos, &second_cell)) = second {
                if Self::can_swap(first_pos, first_cell, second_pos, second_cell) {
                    return Some(self.swap(first_pos, first_cell, second_pos, second_cell));
                }
            }
        }
        None
    }

    pub fn try_undo_swap(&mut self, record: SwapRecord) -> bool {
        let first = self.grid.get(record.a);
        let second = self.grid.get(record.b);
        if let Some(&first_cell) = first {
            if let Some(&second_cell) = second {
                self.undo_swap(record, first_cell, second_cell);
                return true;
            }
        }
        false
    }

    fn compute_fill(&mut self) {
        self.grid.iter_mut().for_each(|(_, cell)| cell.set_min_fill());
        let mut to_spread: VecDeque<Pos2> = self.grid.iter().filter(|(_, cell)| cell.source().is_some())
            .map(|(pos, _)| *pos).collect();
        while let Some(to_check) = to_spread.pop_front() {
            if let Some(cell) = self.grid.get(to_check) {
                let fill = cell.fill();
                for direction in cell.connections().iter() {
                    let neighbor_pos = to_check + direction.to_vec();
                    if let Some(neighbor_cell) = self.grid.get_mut(neighbor_pos) {
                        if neighbor_cell.connections()[direction.inverse()] {
                            let prev: super::ColorSet = neighbor_cell.fill();
                            neighbor_cell.set_fill(prev.union(fill));
                            if neighbor_cell.fill() != prev {
                                to_spread.push_back(neighbor_pos);
                            }
                        }
                    }
                }
            }
        }
    }

    fn all_connected(&self) -> bool {
        self.grid.iter().all(|(pos, cell)| {
            cell.connections().iter().all(|dir| {
                let neighbor_pos = pos + dir.to_vec();
                match self.grid.get(neighbor_pos) {
                    Some(cell) => cell.connections()[dir.inverse()],
                    None => false,
                }
            })
        })
    }

    fn fill_violation(&self) -> Option<PuzzleSolveState> {
        let mut double_filled = false;
        for (pos, cell) in self.grid.iter() {
            if cell.fill().is_empty() {
                return Some(PuzzleSolveState::NotAllFilled)
            }
            if cell.fill().iter().take(2).count() == 2 {
                double_filled = true;
            }
        }
        if double_filled {
            Some(PuzzleSolveState::DoubleFilled)
        }
        else {
            None
        }
    }

    fn single_groups(&self) -> bool {
        Color::ALL.iter().all(|color| {
            let sources: Vec<_> = self.grid.iter()
                .filter(|(_, cell)| cell.source() == Some(*color))
                .map(|(p, _)| p).collect();
            if sources.len() > 1 {
                let mut checked = HashSet::new();
                let mut to_check = vec![*sources[0]];
                while let Some(check) = to_check.pop() {
                    if checked.contains(&check) {
                        continue;
                    }
                    checked.insert(check);
                    if let Some(cell) = self.grid.get(check) {
                        for direction in cell.connections().iter() {
                            let neighbor_pos = check + direction.to_vec();
                            if let Some(neighbor_cell) = self.grid.get(neighbor_pos) {
                                if neighbor_cell.connections()[direction.inverse()] {
                                    to_check.push(neighbor_pos);
                                }
                            }
                        }
                    }
                }
                sources.iter().all(|p| checked.contains(*p))
            }
            else {
                true
            }
        })
    }

    pub fn is_solved(&self) -> PuzzleSolveState {
        if !self.all_connected() {
            return PuzzleSolveState::NotAllConnected
        }
        if let Some(fill_violation) = self.fill_violation() {
            return fill_violation
        }
        if !self.single_groups() {
            return PuzzleSolveState::DuplicateColorSection
        }
        return PuzzleSolveState::Solved;
    }
}