use std::collections::VecDeque;

use egui::ahash::{HashMap, HashMapExt, HashSet};
use rand::seq::IteratorRandom;

use crate::{gameplay::{Color, LayerConnection, PuzzleCell}, grids::{Direction, DirectionMap, DirectionSet, Grid, GridIndex}};

use super::{solutions::{GeneratorCell, GeneratorFailure}, GeneratorSettings};

#[derive(Debug, Clone)]
enum ConnectionCell {
    Single(DirectionSet, Color),
    Intersection(DirectionMap<LayerConnection>, [Color; 2])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CxnGridIndex {
    grid_index: GridIndex,
    layer_index: Option<usize>
}

type ConnectionGrid = Grid<ConnectionCell>;

fn connect_direction(grid: &mut ConnectionGrid, a: CxnGridIndex, dir: Direction) {
    match grid.get_mut(a.grid_index).unwrap() {
        ConnectionCell::Single(connections, _color) => {
            assert!(a.layer_index.is_none());
            connections.insert(dir);
        },
        ConnectionCell::Intersection(connections, _color) => {
            let layer = a.layer_index.unwrap();
            let layer_cxn = if layer == 0 { LayerConnection::Layer0 } else { LayerConnection::Layer1 };
            connections[dir] = layer_cxn
        },
    }
}

fn disconnect_direction(grid: &mut ConnectionGrid, a: CxnGridIndex, dir: Direction) {
    match grid.get_mut(a.grid_index).unwrap() {
        ConnectionCell::Single(connections, _color) => {
            assert!(a.layer_index.is_none());
            connections.remove(dir);
        },
        ConnectionCell::Intersection(connections, _color) => {
            let layer = a.layer_index.unwrap();
            let layer_cxn = if layer == 0 { LayerConnection::Layer0 } else { LayerConnection::Layer1 };
            assert_eq!(connections[dir], layer_cxn);
            connections[dir] = LayerConnection::None;
        },
    }
}

fn connect(grid: &mut ConnectionGrid, a: CxnGridIndex, b: CxnGridIndex, a_to_b: Direction) {
    connect_direction(grid, a, a_to_b);
    connect_direction(grid, b, a_to_b.inverse());
}

fn disconnect(grid: &mut ConnectionGrid, a: CxnGridIndex, b: CxnGridIndex, a_to_b: Direction) {
    disconnect_direction(grid, a, a_to_b);
    disconnect_direction(grid, b, a_to_b.inverse());
}

fn get_all_indices(grid: &ConnectionGrid, match_color: Color) -> impl Iterator<Item = CxnGridIndex> + '_ {
    grid.iter().flat_map(move |(grid_index, cell)| match cell {
        ConnectionCell::Single(_, color) if *color == match_color => vec![CxnGridIndex { grid_index, layer_index: None }],
        ConnectionCell::Single(_, _) => vec![],
        ConnectionCell::Intersection(_, colors) => colors.iter().enumerate()
            .filter_map(|(i, color)| (*color == match_color).then_some(CxnGridIndex { grid_index, layer_index: Some(i) }))
            .collect(),
    })
}

fn get_layer(grid: &ConnectionGrid, index: CxnGridIndex) -> DirectionSet {
    match grid.get(index.grid_index).unwrap() {
        ConnectionCell::Single(cxs, _) => *cxs,
        ConnectionCell::Intersection(cxs, _) => {
            let mut set = DirectionSet::empty();
            let layer = match index.layer_index {
                Some(1) => LayerConnection::Layer1,
                _ => LayerConnection::Layer0
            };
            for entry in cxs.iter() {
                if *entry.1 == layer {
                    set.insert(entry.0);
                }
            }
            set
        },
    }
}

pub fn connect_groups(gen_grid: Grid<GeneratorCell>, generator_settings: &GeneratorSettings) -> Result<Grid<PuzzleCell>, GeneratorFailure> {
    let mut grid = ConnectionGrid::with_size(gen_grid.size());
    let mut intersections_left = generator_settings.max_intersections;
    for (pos, cell) in gen_grid {
        grid.insert(pos, match cell {
            GeneratorCell::SingleGroup(color) => ConnectionCell::Single(DirectionSet::empty(), color),
            GeneratorCell::Intersection(color_a, color_b) => {
                intersections_left -= 1;
                ConnectionCell::Intersection(DirectionMap::new_with_repeat(LayerConnection::None), [color_a, color_b])
            },
        }).unwrap();
    }

    let mut present_colors = Vec::new();
    for color in Color::ALL {
        let indices: HashSet<_> = get_all_indices(&grid, color).collect();
        if !indices.is_empty() {
            present_colors.push(color);
        }
        else {
            continue;
        }

        for index in indices.iter() {
            let neighbors: Vec<_> = grid.iter_neighbors(index.grid_index).map(|n| (n.0, n.1)).collect();
            for neighbor in neighbors {
                for layer_index in [None, Some(0), Some(1)] {
                    let neighbor_index = CxnGridIndex { grid_index: neighbor.0, layer_index };
                    if indices.contains(&neighbor_index) {
                        connect(&mut grid, *index, neighbor_index, neighbor.1)
                    }
                }
            }
        }
    }

    for color in present_colors.iter() {
        knockout_loops(&mut grid, *color, &mut intersections_left, generator_settings);
    }

    let mut all_sources = HashMap::new();
    for color in present_colors.iter() {
        let mut potential_sources: Vec<_> = get_all_indices(&grid, *color)
            .filter(|idx| idx.layer_index.is_none())
            .collect();
        if potential_sources.is_empty() {
            return Err(GeneratorFailure::CannotAddSource);
        }

        let source = potential_sources.swap_remove(random_index(potential_sources.len()));
        all_sources.insert(source.grid_index, *color);
        while !potential_sources.is_empty() && chance(generator_settings.extra_source_chance) {
            all_sources.insert(potential_sources.swap_remove(random_index(potential_sources.len())).grid_index, *color);
        }
    }

    let mut puzzle_grid = Grid::with_size(grid.size());
    for (grid_index, cell) in grid.into_iter() {
        puzzle_grid.insert(grid_index, match cell {
            ConnectionCell::Single(connections, _color) => {
                if let Some(&source) = all_sources.get(&grid_index) {
                    PuzzleCell::Source { connections, source }
                }
                else {
                    PuzzleCell::Normal { connections }
                }
            },
            ConnectionCell::Intersection(connections, _colors) => {
                PuzzleCell::Intersection { connections }
            }
        }).unwrap();
    }
    Ok(puzzle_grid)
}

fn knockout_loops(grid: &mut ConnectionGrid, color: Color, intersections_left: &mut usize, generator_settings: &GeneratorSettings) {
    loop {
        if !chance(generator_settings.knockout_loop_chance) {
            break;
        }

        let indices: HashSet<_> = get_all_indices(grid, color).collect();
        if indices.len() <= 3 {
            break;
        }

        let origin = *indices.iter().choose(&mut rand::thread_rng()).unwrap();
        let mut not_checked = indices;

        let mut counts = HashMap::new();
        counts.insert(origin, 1);
        let mut prevs = HashMap::new();
        let mut to_explore = VecDeque::new();
        to_explore.push_back(origin);

        while let Some(exploring) = to_explore.pop_front() {
            if !not_checked.remove(&exploring) { continue }
            let layer = get_layer(grid, exploring);

            for neighbor in grid.iter_neighbors_for(exploring.grid_index, layer.iter_set()) {
                let is_intersection = match neighbor.2 {
                    ConnectionCell::Single(_, _) => false,
                    ConnectionCell::Intersection(_, _) => true,
                };
                for layer_index in [None, Some(0), Some(1)] {
                    if layer_index.is_some() != is_intersection { continue; }
                    let neighbor_index = CxnGridIndex { grid_index: neighbor.0, layer_index };
                    if not_checked.contains(&neighbor_index) && get_layer(grid, neighbor_index).contains(neighbor.1.inverse()) {
                        match counts.entry(neighbor_index) {
                            std::collections::hash_map::Entry::Occupied(mut entry) => *entry.get_mut() += 1,
                            std::collections::hash_map::Entry::Vacant(entry) => { entry.insert(1); },
                        }
                        prevs.insert(neighbor_index, (exploring, neighbor.1));
                        to_explore.push_back(neighbor_index);
                    }
                }
            }
        }

        if let Some((break_to, (break_from, direction))) = counts.iter()
            .filter(|(_, count)| **count > 1)
            .filter_map(|(p, _)| prevs.get(p).map(|prev| (*p, *prev)))
            .choose(&mut rand::thread_rng()) {
            if break_to.layer_index.is_none() && *intersections_left > 0 && chance(generator_settings.intersection_chance) {
                *intersections_left -= 1;
                let layer = get_layer(grid, break_to);
                let mut connections = DirectionMap::new_with_repeat(LayerConnection::None);
                for dir in layer.iter_set() {
                    if dir == direction.inverse() {
                        connections[dir] = LayerConnection::Layer1;
                    }
                    else {
                        connections[dir] = LayerConnection::Layer0;
                    }
                }
                *grid.get_mut(break_to.grid_index).unwrap() = ConnectionCell::Intersection(connections, [color, color])
            }
            else {
                disconnect(grid, break_from, break_to, direction)
            }
        }
        else {
            break
        }
    }
}

fn random_index(length: usize) -> usize {
    rand::random::<usize>() % length
}

fn chance(c: f32) -> bool {
    rand::random::<f32>() < c
}