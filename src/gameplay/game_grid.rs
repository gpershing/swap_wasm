use std::collections::VecDeque;

use egui::ahash::{HashMap, HashMapExt, HashSet};

use crate::grids::{Grid, GridIndex};

use super::{cell::CellLayer, Cell, CellIdProvider, PuzzleCell};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
enum GameGridLayerIndex {
    Layer0,
    Layer1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GameGridIndex {
    grid_index: GridIndex,
    layer_index: usize
}

pub trait GameGrid {
    fn from_puzzle_grid(puzzle_grid: Grid<PuzzleCell>) -> Self;

    fn is_solved(&self) -> PuzzleSolveState;

    fn swap_with_rotation(&mut self, a: GridIndex, b: GridIndex) -> bool;
    fn fill(&mut self);

    fn get_layer(&self, index: GameGridIndex) -> Option<&CellLayer>;
    fn get_layer_mut(&mut self, index: GameGridIndex) -> Option<&mut CellLayer>;

    fn iter_layers(&self) -> impl Iterator<Item = (GameGridIndex, &CellLayer)>;
    fn iter_layers_at(&self, index: GridIndex) -> Option<impl Iterator<Item = (GameGridIndex, &CellLayer)>>;

    fn iter_connected_layers(&self, index: GameGridIndex) -> Option<impl Iterator<Item = (GameGridIndex, &CellLayer)>>;
    fn all_connected(&self, index: GameGridIndex) -> Option<HashSet<GameGridIndex>>;
}

fn can_swap(grid: &Grid<Cell>, a: GridIndex, b: GridIndex) -> bool {
    if a == b {
        return false
    }
    if let Some(cell_a) = grid.get(a) {
        if let Some(cell_b) = grid.get(b) {
            return Cell::can_swap(cell_a, cell_b)
        }
    }
    false
}

impl GameGrid for Grid<Cell> {
    fn from_puzzle_grid(puzzle_grid: Grid<PuzzleCell>) -> Self {
        let mut grid = Self::with_size(puzzle_grid.size());
        let mut ids = CellIdProvider::new();
        for (pos, cell) in puzzle_grid.into_iter() {
            grid.insert(pos, Cell::new(ids.next(), cell));
        }
        grid.fill();
        grid
    }

    fn is_solved(&self) -> PuzzleSolveState {
        let all_connected = self.iter_layers().all(|(index, layer)|
            self.iter_connected_layers(index).unwrap().count() == layer.connections.len());
        if !all_connected { return PuzzleSolveState::NotAllConnected }

        let all_filled = self.iter_layers().all(|(index, layer)| !layer.fill.is_empty());
        if !all_filled { return PuzzleSolveState::NotAllFilled }

        let double_filled = self.iter_layers().any(|(index, layer)| layer.fill.iter().take(2).count() == 2);
        if double_filled { return PuzzleSolveState::DoubleFilled }

        fn has_duplicate_fill(slf: &Grid<Cell>) -> bool {
            let mut source_counts = HashMap::new();
            for cell in slf.iter() {
                if let Some(source) = cell.1.source() {
                    match source_counts.entry(source) {
                        std::collections::hash_map::Entry::Occupied(entry) => entry.get_mut() += 1,
                        std::collections::hash_map::Entry::Vacant(entry) => entry.insert(1),
                    }
                }
            }
            for (color, count) in source_counts.into_iter().filter(|(_, count)| *count > 1) {
                let any_source = slf.iter().find(|(_, cell)| cell.source() == Some(color)).unwrap();
                let all_connected = slf.all_connected(GameGridIndex { grid_index: any_source.0, layer_index: 0 }).unwrap();
                // Assume that double_filled is false
                let source_count = all_connected.iter().filter_map(|index| slf.get(index.grid_index).and_then(|c| c.source()))
                    .count();
                if source_count != count {
                    return true
                }
            }
            false
        }
        if has_duplicate_fill(&self) { return PuzzleSolveState::DuplicateColorSection }

        return PuzzleSolveState::Solved;
    }

    fn get_layer(&self, index: GameGridIndex) -> Option<&CellLayer> {
        let cell = self.get(index.grid_index)?;
        cell.get_layer(index.layer_index.into())
    }

    fn get_layer_mut(&mut self, index: GameGridIndex) -> Option<&mut CellLayer> {
        let cell = self.get_mut(index.grid_index)?;
        cell.get_layer_mut(index.layer_index.into())
    }

    fn swap_with_rotation(&mut self, a: GridIndex, b: GridIndex) -> bool {
        match can_swap(&self, a, b) {
            true => {
                unsafe {
                    self.get_unchecked_mut(a).unwrap_unchecked().rotate_by_fill();
                    self.get_unchecked_mut(b).unwrap_unchecked().rotate_by_fill();
                }
                self.swap(a, b);
                true
            },
            false => false,
        }
    }

    fn fill(&mut self) {
        self.iter_mut().for_each(|c| c.1.clear_fill());
        let mut to_explore: VecDeque<_> = self.iter().filter_map(|(grid_index, cell)| {
            cell.source().map(|source| (GameGridIndex { grid_index, layer_index: 0 }, source))
        }).collect();
        while let Some((to_explore, to_fill)) = to_explore.pop_front() {
            let cell = self.get_layer_mut(to_explore).unwrap();
            let union = cell.fill.union(to_fill);
            if union != cell.fill {
                cell.fill = union;
                for (neighbor, layer) in self.iter_connected_layers(to_explore).unwrap() {
                    if union != layer.fill {
                        to_explore.push_back((neighbor, union))
                    }
                }
            }
        }
    }

    fn iter_layers(&self) -> impl Iterator<Item = (GameGridIndex, &CellLayer)> {
        self.iter()
            .map(|c| c.1.iter_layers()
                .enumerate()
                .map(|(idx, layer)| (GameGridIndex { grid_index: c.0, layer_index: idx })))
            .flatten()
    }

    fn iter_layers_at(&self, index: GridIndex) -> Option<impl Iterator<Item = (GameGridIndex, &CellLayer)>> {
        self.get(index).map(|cell|
            cell.iter_layers()
                .enumerate()
                .map(|(idx, layer)| GameGridIndex { grid_index: index, layer_index: idx }))
    }

    fn iter_connected_layers(&self, index: GameGridIndex) -> Option<impl Iterator<Item = (GameGridIndex, &CellLayer)>> {
        let layer = self.get_layer(index)?;
        Some(self.iter_neighbors_for(index, layer.connections.iter_set())
                    .filter_map(|(_idx, direction, cell)| cell.get_layer_for_direction(direction.inverse())))
    }

    fn all_connected(&self, index: GameGridIndex) -> Option<HashSet<GameGridIndex>> {
        if self.get(index).is_none() { return None }
        let explored = HashSet::new();
        let to_explore = Vec::new();
        to_explore.push(index);
        while let Some(exploring) = to_explore.pop() {
            explored.insert(exploring);
            for layer in self.iter_connected_layers(index).unwrap() {
                if !explored.contains(&layer.0) {
                    to_explore.push(layer.0)
                }
            }
        }
        Some(explored)
    }
}