use egui::ahash::{HashSet, HashSetExt};
use rand::seq::IteratorRandom;

use crate::{gameplay::{Color, GameGrid, GridSolveState, PuzzleCell}, generator::connections::connect_groups, grids::{Grid, GridIndex}};

use super::GeneratorSettings;

pub fn generate_solution(generator_settings: &GeneratorSettings) -> Grid<PuzzleCell> {
    loop {
        match try_generate_solution(generator_settings) {
            Ok(solution) => break solution,
            Err(err) => println!("{err:?} (retrying)"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum GeneratorCell {
    SingleGroup(Color),
    Intersection(Color, Color)
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum GeneratorFailure {
    RandomFailure,
    ResultNotSolved,
    CannotAddSource,
}

fn try_generate_solution(generator_settings: &GeneratorSettings) -> Result<Grid<PuzzleCell>, GeneratorFailure> {
    let grid = create_grid_with_knockouts(generator_settings);
    println!("grid created");
    let grid = allocate_groups(grid, generator_settings)?;
    println!("grid allocated");
    let grid = connect_groups(grid, generator_settings)?;
    println!("grid connected");
    verify(grid)
}

fn create_grid_with_knockouts(generator_settings: &GeneratorSettings) -> Grid<()> {
    let mut grid = Grid::with_size(generator_settings.size);
    let mut positions: Vec<_> = grid.size().into_iter().collect();

    let mut knockouts = 0;
    while knockouts < generator_settings.missing && !positions.is_empty() {
        if chance(generator_settings.missing_chance) {
            knockouts += 1;
            positions.swap_remove(random_index(positions.len()));
        }
    }

    for position in positions {
        grid.insert(position, ()).unwrap();
    }
    grid
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum GroupStatus {
    Happy,
    MaxIntersections,
    TooSmall,
}

#[derive(Debug, Clone)]
struct Group {
    color: Color,
    present: HashSet<GridIndex>,
    boundary: HashSet<GridIndex>,
    intersections: usize
}

impl Group {
    fn new(color: Color) -> Self {
        Self { color, present: HashSet::new(), boundary: HashSet::new(), intersections: 0 }
    }

    fn status(&self) -> GroupStatus {
        let required = match self.color {
            Color::Red | Color::Orange | Color::Yellow | Color::Purple => 2,
            Color::Green | Color::Blue => 1
        };
        if required > self.present.len() {
            GroupStatus::TooSmall
        }
        else if self.intersections >= self.present.len() {
            GroupStatus::MaxIntersections
        }
        else {
            GroupStatus::Happy
        }
    }
}

fn allocate_groups(grid: Grid<()>, generator_settings: &GeneratorSettings) -> Result<Grid<GeneratorCell>, GeneratorFailure> {
    fn get_group_colors(generator_settings: &GeneratorSettings) -> Vec<Color> {
        let mut groups = vec![Color::Purple];
        let mut possible_groups = vec![Color::Blue, Color::Green];
        match generator_settings.stop_sources {
            super::SourceSettings::None => (),
            super::SourceSettings::Maybe => possible_groups.push(Color::STOP),
            super::SourceSettings::Definitely => groups.push(Color::STOP),
        }
        match generator_settings.rotator_sources {
            super::SourceSettings::None => (),
            super::SourceSettings::Maybe => possible_groups.extend([Color::CCW, Color::CW]),
            super::SourceSettings::Definitely => {
                if chance(0.5) {
                    groups.push(Color::CCW);
                    possible_groups.push(Color::CW);
                }
                else {
                    groups.push(Color::CW);
                    possible_groups.push(Color::CCW);
                }
            },
        }
        while !possible_groups.is_empty() {
            let mut add = groups.len() < generator_settings.min_regions + 1;
            if !add { add = chance(generator_settings.extra_region_chance) }
            if !add { break }
    
            groups.push(possible_groups.swap_remove(random_index(possible_groups.len())));
        }
        groups
    }
    let mut groups: Vec<_> = get_group_colors(generator_settings).into_iter()
        .map(Group::new)
        .collect();
    let mut group_grid = Grid::with_size(grid.size());
    let mut intersections_left = generator_settings.max_intersections;
    
    while group_grid.len() < grid.len() {
        groups.sort_by_cached_key(|_| rand::random::<u32>());
        groups.sort_by_key(|g| g.status());
        if let Some(group) = groups.last() {
            let add = if group.present.is_empty() {
                let position = grid.iter().map(|(p, _)| p).filter(|p| !group_grid.contains(*p)).choose(&mut rand::thread_rng()).unwrap();
                Ok(position)
            }
            else if let Some(&position) = group.boundary.iter().choose(&mut rand::thread_rng()) {
                let add = match group_grid.get(position) {
                    Some(GeneratorCell::SingleGroup(in_group)) => {
                        *in_group != group.color && intersections_left > 0 && chance(generator_settings.intersection_chance)
                    },
                    Some(GeneratorCell::Intersection(_, _)) => false,
                    None => true,
                };
                if add {
                    Ok(position)
                }
                else {
                    Err(Some(position))
                }
            }
            else {
                if group.status() != GroupStatus::Happy {
                    return Err(GeneratorFailure::RandomFailure)
                }
                groups.pop();
                Err(None)
            };
            if let Ok(position) = add {
                let group = groups.last_mut().unwrap();
                group.present.insert(position);
                group.boundary.remove(&position);
                group.boundary.extend(grid.iter_neighbors(position).map(|(p, _, _)| p));

                let intersection = match group_grid.get(position) {
                    Some(GeneratorCell::SingleGroup(in_group)) => Some(in_group),
                    Some(GeneratorCell::Intersection(_, _)) => panic!(),
                    None => None,
                }.cloned();

                if let Some(intersect) = intersection {
                    intersections_left -= 1;
                    *group_grid.get_mut(position).unwrap() = GeneratorCell::Intersection(intersect, group.color);
                    group.intersections += 1;
                    if let Some(other_group) = groups.iter_mut().find(|group| group.color == intersect) {
                        other_group.intersections += 1;
                    }
                }
                else {
                    group_grid.insert(position, GeneratorCell::SingleGroup(group.color)).unwrap();
                }
            }
            else if let Err(Some(failure)) = add {
                let group = groups.last_mut().unwrap();
                group.boundary.remove(&failure);
            }
        }
        else {
            break
        }
    }

    Ok(group_grid)
}

fn verify(grid: Grid<PuzzleCell>) -> Result<Grid<PuzzleCell>, GeneratorFailure> {
    let game_grid = Grid::from_puzzle_grid(grid.clone());
    if game_grid.is_solved() != GridSolveState::Solved {
        return Err(GeneratorFailure::ResultNotSolved)
    }
    Ok(grid)
}

fn random_index(length: usize) -> usize {
    rand::random::<usize>() % length
}

fn chance(c: f32) -> bool {
    rand::random::<f32>() < c
}