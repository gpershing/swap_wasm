use egui::ahash::{HashSet, HashSetExt};

use crate::grid_math::{Pos2, Rotation, Rect};

use super::{game_grid::GameGrid, Cell, CellId, Color, ColorSet, Puzzle, SwapRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub enum PuzzleSolveState {
    Solved,
    TooManySwaps,
    NotAllConnected,
    NotAllFilled,
    DoubleFilled,
    DuplicateColorSection
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlayingPuzzle {
    puzzle: Puzzle,
    grid: GameGrid,
    history: Vec<SwapRecord>
}

impl PlayingPuzzle {
    pub fn new(puzzle: Puzzle) -> PlayingPuzzle {
        let grid = GameGrid::from_puzzle_grid(puzzle.start());
        Self { 
            puzzle,
            grid,
            history: Vec::new()
        }
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = (&Pos2, &Cell)> {
        self.grid.iter()
    }
    
    fn get(&self, pos: Pos2) -> Option<&Cell> {
        self.grid.get(pos)
    }

    pub fn swaps_made(&self) -> usize {
        self.history.len()
    }

    pub fn swap_limit(&self) -> usize {
        self.puzzle.swap_limit() as usize
    }

    pub fn id_for_position(&self, pos: Pos2) -> Option<CellId> {
        self.get(pos).map(|cell| cell.id())
    }

    fn can_swap(first_pos: Pos2, first_cell: Cell, second_pos: Pos2, second_cell: Cell) -> bool {
        first_pos != second_pos && Cell::can_swap(&first_cell, &second_cell)
    }

    fn swap(&mut self, first_pos: Pos2, first_cell: Cell, second_pos: Pos2, second_cell: Cell) -> SwapRecord {
        let first_rotation = first_cell.fill().get_rotation();
        let second_rotation = second_cell.fill().get_rotation();
        self.grid.swap(first_pos, first_rotation, first_cell, second_pos, second_rotation, second_cell);
        SwapRecord::new(first_pos, second_pos, first_rotation, second_rotation)
    }

    fn try_swap_record(&mut self, first: CellId, second: CellId) -> Option<SwapRecord> {
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

    pub fn try_swap(&mut self, id_a: CellId, id_b: CellId) -> bool {
        match self.try_swap_record(id_a, id_b) {
            Some(record) => {
                self.history.push(record);
                true
            }
            None => false,
        }
    }

    fn undo_swap(&mut self, record: SwapRecord, first_cell: Cell, second_cell: Cell) {
        self.grid.swap(record.a, record.b_rotation.inverse(), first_cell, record.b, record.a_rotation.inverse(), second_cell);
    }

    fn try_undo_record(&mut self, record: SwapRecord) -> bool {
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

    pub fn try_undo(&mut self) -> bool {
        match self.history.pop() {
            Some(record) => self.try_undo_record(record),
            None => false,
        }
    }

    pub fn reset(&mut self) {
        self.grid = GameGrid::from_puzzle_grid(self.puzzle.start());
        self.history.clear();
    }

    pub fn bounds(&self) -> Rect {
        self.grid.bounds()
    }
    
    fn entry_for_id(&self, id: CellId) -> Option<(&Pos2, &Cell)> {
        self.grid.iter().find(|(_pos, cell)| cell.id() == id)
    }

    pub fn is_solved(&self) -> PuzzleSolveState {
        if self.swaps_made() > self.swap_limit() as usize {
            return PuzzleSolveState::TooManySwaps
        }
        return self.grid.is_solved()
    }
}