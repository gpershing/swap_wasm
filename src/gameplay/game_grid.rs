use std::collections::VecDeque;

use egui::ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

use crate::grids::{Grid, GridIndex, Rotation};

use super::{cell::CellLayer, Cell, ColorSet, PuzzleCell, SwapRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(serde::Serialize, serde:: Deserialize)]
pub enum GridSolveState {
    Solved,
    NotAllConnected,
    NotAllFilled,
    DoubleFilled,
    DuplicateColorSection
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GameGridIndex {
    grid_index: GridIndex,
    layer_index: usize
}

pub trait GameGrid {
    fn from_puzzle_grid(puzzle_grid: Grid<PuzzleCell>) -> Self;

    fn is_solved(&self) -> GridSolveState;

    fn swap_with_rotation(&mut self, a: GridIndex, b: GridIndex) -> Option<SwapRecord>;
    fn undo_swap(&mut self, record: SwapRecord);
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
        for (pos, cell) in puzzle_grid.into_iter() {
            grid.insert(pos, Cell::new(cell)).unwrap();
        }
        grid.fill();
        grid
    }

    fn is_solved(&self) -> GridSolveState {
        let all_connected = self.iter_layers().all(|(index, layer)|
            self.iter_connected_layers(index).unwrap().count() == layer.connections.len());
        if !all_connected { return GridSolveState::NotAllConnected }

        let all_filled = self.iter_layers().all(|(_index, layer)| !layer.fill.is_empty());
        if !all_filled { return GridSolveState::NotAllFilled }

        let double_filled = self.iter_layers().any(|(_index, layer)| layer.fill.iter().take(2).count() == 2);
        if double_filled { return GridSolveState::DoubleFilled }

        fn has_duplicate_fill(slf: &Grid<Cell>) -> bool {
            let mut source_counts = HashMap::new();
            for cell in slf.iter() {
                if let Some(source) = cell.1.source() {
                    match source_counts.entry(source) {
                        std::collections::hash_map::Entry::Occupied(mut entry) => *entry.get_mut() += 1,
                        std::collections::hash_map::Entry::Vacant(entry) => { entry.insert(1); },
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
        if has_duplicate_fill(self) { return GridSolveState::DuplicateColorSection }

        GridSolveState::Solved
    }

    fn get_layer(&self, index: GameGridIndex) -> Option<&CellLayer> {
        let cell = self.get(index.grid_index)?;
        cell.get_layer(index.layer_index)
    }

    fn get_layer_mut(&mut self, index: GameGridIndex) -> Option<&mut CellLayer> {
        let cell = self.get_mut(index.grid_index)?;
        cell.get_layer_mut(index.layer_index)
    }

    fn swap_with_rotation(&mut self, a: GridIndex, b: GridIndex) -> Option<SwapRecord> {
        match can_swap(self, a, b) {
            true => {
                let a_rotation: Rotation;
                let b_rotation: Rotation;
                unsafe {
                    a_rotation = self.get_unchecked_mut(a).unwrap_unchecked().rotate_by_fill();
                    b_rotation = self.get_unchecked_mut(b).unwrap_unchecked().rotate_by_fill();
                }
                self.swap(a, b);
                Some(SwapRecord { a, b, a_rotation, b_rotation })
            },
            false => None,
        }
    }

    fn undo_swap(&mut self, record: SwapRecord) {
        self.swap(record.a, record.b);
        self.get_mut(record.a).unwrap().rotate(record.a_rotation.inverse());
        self.get_mut(record.b).unwrap().rotate(record.b_rotation.inverse());
    }

    fn fill(&mut self) {
        self.iter_mut().for_each(|c| c.1.clear_fill());
        let mut to_explore_queue: VecDeque<_> = self.iter().filter_map(|(grid_index, cell)| {
            cell.source().map(|source| (GameGridIndex { grid_index, layer_index: 0 }, ColorSet::singleton(source)))
        }).collect();
        while let Some((to_explore, to_fill)) = to_explore_queue.pop_front() {
            let cell = self.get_layer_mut(to_explore).unwrap();
            let union = cell.fill.union(to_fill);
            if union != cell.fill {
                cell.fill = union;
                for (neighbor, layer) in self.iter_connected_layers(to_explore).unwrap() {
                    if union != layer.fill {
                        to_explore_queue.push_back((neighbor, union))
                    }
                }
            }
        }
    }

    fn iter_layers(&self) -> impl Iterator<Item = (GameGridIndex, &CellLayer)> {
        self.iter()
            .flat_map(|c| c.1.iter_layers()
                .enumerate()
                .map(move |(idx, layer)| (GameGridIndex { grid_index: c.0, layer_index: idx }, layer)))
    }

    fn iter_layers_at(&self, index: GridIndex) -> Option<impl Iterator<Item = (GameGridIndex, &CellLayer)>> {
        self.get(index).map(move |cell|
            cell.iter_layers()
                .enumerate()
                .map(move |(idx, layer)| (GameGridIndex { grid_index: index, layer_index: idx }, layer)))
    }

    fn iter_connected_layers(&self, index: GameGridIndex) -> Option<impl Iterator<Item = (GameGridIndex, &CellLayer)>> {
        let layer = self.get_layer(index)?;
        Some(self.iter_neighbors_for(index.grid_index, layer.connections.iter_set())
                    .filter_map(|(grid_index, direction, cell)| cell
                        .get_layer_for_direction(direction.inverse())
                        .map(|(layer_index, cell)| (GameGridIndex { grid_index, layer_index }, cell))))
    }

    fn all_connected(&self, index: GameGridIndex) -> Option<HashSet<GameGridIndex>> {
        _ = self.get(index.grid_index)?;
        let mut explored = HashSet::new();
        let mut to_explore = Vec::new();
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