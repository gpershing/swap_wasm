use std::collections::VecDeque;

use egui::ahash::{HashSet, HashSetExt};

use crate::grid_math::{Grid, Pos2, Rect, Rotation};

use super::{puzzle::PuzzleCell, Cell, CellIdProvider, Color, PuzzleSolveState};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GameGrid(Grid<Cell>);

pub fn puzzle_grid_to_playing_grid(puzzle_grid: Grid<PuzzleCell>) -> Grid<Cell> {
    let mut ids = CellIdProvider::new();
    let mut grid = Grid::new();
    for (pos, cell) in puzzle_grid.into_iter() {
        grid.insert(pos, Cell::new(ids.next(), cell));
    }
    fill_playing_grid(&mut grid);
    grid
}

pub fn fill_playing_grid(grid: &mut Grid<Cell>) {
    grid.iter_mut().for_each(|(_, cell)| cell.set_min_fill());
    let mut to_spread: VecDeque<_> = grid.iter().filter(|(_, cell)| cell.source().is_some())
        .map(|(pos, _)| *pos).collect();
    while let Some(to_check) = to_spread.pop_front() {
        if let Some(cell) = grid.get(to_check) {
            let fill = cell.fill();
            for direction in cell.connections().iter() {
                let neighbor_pos = to_check + direction.to_vec();
                if let Some(neighbor_cell) = grid.get_mut(neighbor_pos) {
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

fn all_connected(grid: &Grid<Cell>) -> bool {
    grid.iter().all(|(pos, cell)| {
        cell.connections().iter().all(|dir| {
            let neighbor_pos = pos + dir.to_vec();
            match grid.get(neighbor_pos) {
                Some(cell) => cell.connections()[dir.inverse()],
                None => false,
            }
        })
    })
}

fn fill_violation(grid: &Grid<Cell>) -> Option<PuzzleSolveState> {
    let mut double_filled = false;
    for (_pos, cell) in grid.iter() {
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

fn single_groups(grid: &Grid<Cell>) -> bool {
    Color::ALL.iter().all(|color| {
        let sources: Vec<_> = grid.iter()
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
                if let Some(cell) = grid.get(check) {
                    for direction in cell.connections().iter() {
                        let neighbor_pos = check + direction.to_vec();
                        if let Some(neighbor_cell) = grid.get(neighbor_pos) {
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

pub fn is_solved(grid: &Grid<Cell>) -> PuzzleSolveState {
    if !all_connected(grid) {
        return PuzzleSolveState::NotAllConnected
    }
    if let Some(fill_violation) = fill_violation(grid) {
        return fill_violation
    }
    if !single_groups(grid) {
        return PuzzleSolveState::DuplicateColorSection
    }
    return PuzzleSolveState::Solved;
}

impl GameGrid {
    pub fn from_puzzle_grid(grid: Grid<PuzzleCell>) -> Self {
        Self(puzzle_grid_to_playing_grid(grid))
    }

    pub fn swap(&mut self, a: Pos2, rotate_a: Rotation, mut cell_a: Cell, b: Pos2, rotate_b: Rotation, mut cell_b: Cell) {
        cell_a.rotate_connections(rotate_a);
        cell_b.rotate_connections(rotate_b);
        self.0.insert(a, cell_b);
        self.0.insert(b, cell_a);
        fill_playing_grid(&mut self.0);
    }
    
    pub fn iter(&self) -> impl Iterator<Item = (&Pos2, &Cell)> {
        self.0.iter()
    }

    pub fn get(&self, pos: Pos2) -> Option<&Cell> {
        self.0.get(pos)
    }

    pub fn bounds(&self) -> Rect {
        self.0.bounds()
    }

    pub fn is_solved(&self) -> PuzzleSolveState {
        is_solved(&self.0)
    }
}