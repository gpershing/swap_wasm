use crate::grids::{Grid, GridIndex, GridSize};

use super::{game_grid::GridSolveState, Cell, GameGrid, Puzzle, SwapRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub enum PuzzleSolveState {
    Solved,
    NotAllConnected,
    NotAllFilled,
    DoubleFilled,
    DuplicateColorSection,
    TooManySwaps,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlayingPuzzle {
    puzzle: Puzzle,
    grid: Grid<Cell>,
    history: Vec<SwapRecord>
}

impl PlayingPuzzle {
    pub fn play(puzzle: Puzzle) -> Self {
        let grid = Grid::from_puzzle_grid(puzzle.start());
        Self { puzzle, grid, history: Vec::new() }
    }

    pub fn swaps_made(&self) -> usize {
        self.history.len()
    }

    pub fn swap_limit(&self) -> usize {
        self.puzzle.swap_limit() as usize
    }

    pub const fn size(&self) -> GridSize {
        self.grid.size()
    }

    pub fn index_has_cell(&self, grid_index: GridIndex) -> bool {
        self.grid.get(grid_index).is_some()
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = (GridIndex, &Cell)> {
        self.grid.iter()
    }

    pub fn reset(&mut self) {
        self.grid = Grid::from_puzzle_grid(self.puzzle.start());
    }

    pub fn try_swap(&mut self, a: GridIndex, b: GridIndex) -> bool {
        if let Some(record) = self.grid.swap_with_rotation(a, b) {
            self.history.push(record);
            self.grid.fill();
            true
        }
        else {
            false
        }
    }
    
    pub fn try_undo(&mut self) -> bool {
        if let Some(record) = self.history.pop() {
            self.grid.undo_swap(record);
            self.grid.fill();
            true
        }
        else {
            false
        }
    }

    pub fn is_solved(&self) -> PuzzleSolveState {
        if self.swaps_made() > self.swap_limit() {
            return PuzzleSolveState::TooManySwaps;
        }

        match self.grid.is_solved() {
            GridSolveState::Solved => PuzzleSolveState::Solved,
            GridSolveState::NotAllConnected => PuzzleSolveState::NotAllConnected,
            GridSolveState::NotAllFilled => PuzzleSolveState::NotAllFilled,
            GridSolveState::DoubleFilled => PuzzleSolveState::DoubleFilled,
            GridSolveState::DuplicateColorSection => PuzzleSolveState::DuplicateColorSection,
        }
    }
}