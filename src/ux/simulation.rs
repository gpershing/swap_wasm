use egui::{ahash::{HashMap, HashMapExt}, Color32};

use crate::{gameplay::{Cell, Color, FColor, SwapRecord}, grids::{Direction, Grid, GridIndex, Rotation}};

#[derive(Debug, Clone, Copy)]
pub struct SimulationSegment {
    start_index: usize,
    end_index: usize
}

#[derive(Debug, Clone, Copy)]
pub struct SimulationCell {
    previous: usize,
    next: usize
}

pub struct Simulation {
    t: f32,

    indices: Vec<SimulationCell>,
    current_colors: Vec<FColor>,
    current: Vec<[f32; 6]>,
    next: Vec<[f32; 6]>,
    retain: Vec<[f32; 6]>,
    segments: HashMap<(GridIndex, Direction), SimulationSegment>,

    simulated_cell_count: usize,
    void_index: usize
}

impl Simulation {
    const SEGMENT_LENGTH: usize = 6;
    const ALPHA: f32 = 0.15;
    const SOURCE: f32 = 3.0;
    const RETAIN_LOSS: f32 = 0.995;
    pub const DT: f32 = 0.003;

    pub fn new(grid: &Grid<Cell>) -> Self {
        let segment_count: usize = grid.iter().map(|(_, cell)| cell.total_connections()).sum();
        let simulated_cell_count: usize = segment_count * Self::SEGMENT_LENGTH;
        let void_index: usize = simulated_cell_count;
        let color_index_start = void_index + 1;
        let all_cell_count = color_index_start + Color::ALL.len();
        
        let mut segments = HashMap::new();
        let mut cells = [SimulationCell { previous: 0, next: 0 }].repeat(all_cell_count);
        let mut current = [[0.0; 6]].repeat(all_cell_count);
        let mut next = [[0.0; 6]].repeat(all_cell_count);
        let retain = [[1.0; 6]].repeat(simulated_cell_count);
        let mut current_colors = [FColor::rgb(0.0, 0.0, 0.0)].repeat(all_cell_count);

        let mut at = 0;
        let mut inner_connections = Vec::new();
        for (index, cell) in grid.iter() {
            for layer in cell.iter_layers() {
                for direction in layer.connections.iter_set() {
                    let start_index = at;
                    let end_index = at + Self::SEGMENT_LENGTH;
                    at = end_index;

                    segments.insert((index, direction), SimulationSegment { start_index, end_index });

                    for i in start_index..(end_index-1) {
                        cells[i].next = i + 1;
                    }
                    cells[end_index - 1].next = end_index - 1;
                    
                    cells[start_index].previous = void_index;
                    for i in (start_index+1)..end_index {
                        cells[i].previous = i - 1;
                    }

                    if let Some(source) = cell.source() {
                        cells[end_index - 1].next = color_index_start + source.index();
                    }
                    else {
                        let mut check_direction = direction;
                        for _ in 0..3 {
                            check_direction = check_direction.rotated(Rotation::CCW);
                            if layer.connections.contains(check_direction) {
                                inner_connections.push((index, direction, check_direction));
                                break;
                            }
                        }
                    }
                }
            }
        }
        for (index, from, to) in inner_connections {
            let v = segments.get(&(index, to)).unwrap().end_index - 1;
            let i = segments.get_mut(&(index, from)).unwrap().end_index - 1;
            cells[i].next = v;
        }

        for ((index, direction), segment) in segments.iter() {
            if let Some(neighbor_index) = index.moved_in(*direction) {
                if let Some(neighbor) = segments.get(&(neighbor_index, direction.inverse())) {
                    cells[segment.start_index].previous = neighbor.start_index;
                }
            }
        }

        for color in Color::ALL {
            let index: usize = color_index_start + color.index();
            current[index][color.index()] = Self::SOURCE;
            next[index][color.index()] = Self::SOURCE;
            current_colors[index] = color.fcolor();
        }

        Self { t: 0.0, indices: cells, current, next, retain, current_colors, segments, simulated_cell_count, void_index }
    }

    pub fn update_fill(&mut self, grid: &Grid<Cell>) {
        for (index, cell) in grid.iter() {
            if cell.source().is_some() { continue; }
            for direction in Direction::ALL {
                if let Some(segment) = self.segments.get(&(index, direction)) {
                    for i in segment.start_index..segment.end_index {
                        let layer = cell.iter_layers().find(|layer| layer.connections.contains(direction)).unwrap();
                        for color in Color::ALL {
                            self.retain[i][color.index()] = if layer.fill.contains(color) { 1.0 } else { Self::RETAIN_LOSS };
                        }
                    }
                }
            }
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.t += dt;
        while self.t >= Self::DT {
            self.step_dt();
            self.t -= Self::DT;
        }
    }

    fn step_dt(&mut self) {
        for (index, cell_indices) in self.indices.iter().enumerate().take(self.simulated_cell_count) {
            let mut sum = 0.0;
            for i in 0..6 {
                self.next[index][i] = self.current[index][i] * self.retain[index][i] + (self.current[cell_indices.next][i] + self.current[cell_indices.previous][i] - 2.0 * self.current[index][i]) * Self::ALPHA * 0.5;
                sum += self.next[index][i];
            }
            if sum <= 0.001 {
                self.current_colors[index] = FColor::rgb(0.0, 0.0, 0.0);
            }
            else {
                let sum_inv = 1.0 / sum;
                let mut next_color = FColor::rgb(0.0, 0.0, 0.0);
                for color in Color::ALL {
                    let t = self.next[index][color.index()].min(1.0);
                    next_color += color.fcolor() * (t * (2.0 - t)) * self.next[index][color.index()] * sum_inv;
                }
                self.current_colors[index] = next_color;
            }
        }
        std::mem::swap(&mut self.current, &mut self.next);
    }

    // prev
    // first = 3
    // end = 6
    // [] 3 4 5 []
    //   0 1 2 3
    // proj_float in [0.5, 3.5]
    // proj_floor in { 0, 1, 2, 3 }
    // index_t in [0, 1)
    pub fn lerp(&self, first: usize, end: usize, t: f32) -> Color32 {
        let length = end - first;
        let proj_float = t * (length as f32) + 0.5;
        let proj_floor = proj_float.floor();
        let index_t = proj_float - proj_floor;
        let proj_index = proj_floor as usize;

        let (a, b) = if proj_index == 0 {
            (self.indices[first].previous, first)
        } else if proj_index >= length {
            (end - 1, self.indices[end - 1].next)
        } else {
            (first + proj_index - 1, first + proj_index)
        };

        (self.current_colors[a] * (1.0 - index_t) + self.current_colors[b] * index_t).to_color32()
    }

    fn break_neighbor_connection_from(&mut self, index: GridIndex, direction: Direction) {
        if let Some(neighbor_index) = index.moved_in(direction) {
            if let Some(cell) = self.segments.get_mut(&(neighbor_index, direction.inverse())) {
                self.indices[cell.start_index].previous = self.void_index;
            }
        }
    }

    pub fn swap(&mut self, record: SwapRecord) {
        let mut to_add = HashMap::new();
        for (index, rotation, new_index) in [(record.a, record.a_rotation, record.b), (record.b, record.b_rotation, record.a)] {
            for unrotated_direction in Direction::ALL {
                if let Some(segment) = self.segments.remove(&(index, unrotated_direction)) {
                    to_add.insert((new_index, unrotated_direction.rotated(rotation)), segment);
                    self.break_neighbor_connection_from(index, unrotated_direction);
                }
            }
        }

        self.segments.extend(to_add.clone());
        for ((index, direction), segment) in to_add {
            self.indices[segment.start_index].previous = self.void_index;
            if let Some(neighbor_index) = index.moved_in(direction) {
                if let Some(neighbor) = self.segments.get(&(neighbor_index, direction.inverse())) {
                    self.indices[segment.start_index].previous = neighbor.start_index;
                    self.indices[neighbor.start_index].previous = segment.start_index;
                }
            }
        }
    }

    const fn void_segment(&self) -> SimulationSegment {
        SimulationSegment { start_index: self.void_index, end_index: self.void_index + 1 }
    }

    pub fn color_fn_single(&self, a: (GridIndex, Direction)) -> impl Fn(f32) -> Color32 + '_ {
        let cell = self.segments.get(&a).copied().unwrap_or(self.void_segment());
        move |t| {
            self.lerp(cell.start_index, cell.end_index, t)
        }
    }

    pub fn color_fn_single_both_ways(&self, a: (GridIndex, Direction)) -> impl Fn(f32) -> Color32 + '_ {
        let cell = self.segments.get(&a).copied().unwrap_or(self.void_segment());
        move |t| {
            if t > 0.5 {
                self.lerp(cell.start_index, cell.end_index, 2.0 - t * 2.0)
            }
            else {
                self.lerp(cell.start_index, cell.end_index, t * 2.0)
            }
        }
    }

    pub fn color_fn_through_two(&self, a: (GridIndex, Direction), b: (GridIndex, Direction)) -> impl Fn(f32) -> Color32 + '_ {
        let cell_a = self.segments.get(&a).copied().unwrap_or(self.void_segment());
        let cell_b = self.segments.get(&b).copied().unwrap_or(self.void_segment());
        move |t| {
            if t > 0.5 {
                self.lerp(cell_b.start_index, cell_b.end_index, 2.0 - t * 2.0)
            }
            else {
                self.lerp(cell_a.start_index, cell_a.end_index, t * 2.0)
            }
        }
    }
}