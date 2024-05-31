use egui::ahash::{HashMap, HashMapExt};
use rand::prelude::*;
use crate::{gameplay::{Cell, Color, GameGrid, LayerConnection, Puzzle, PuzzleCell, SwapRecord}, grids::{Direction, DirectionMap, DirectionMapData, DirectionSet, Grid, GridIndex, GridSize}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceSettings {
    None,
    Maybe,
    Definitely
}

#[derive(Debug, Clone)]
pub struct GeneratorSettings {
    // Size of the grid.
    pub size: GridSize,
    // Number of swaps.
    pub swap_count: u8,
    // Odds that a single space is missing.
    pub missing_chance: f32,
    // Maximum number of missing spaces.
    pub missing: usize,
    // Can the puzzle include red sources?
    pub red_sources: SourceSettings,
    // Can the puzzle include yellow or orange sources?
    pub rotator_sources: SourceSettings,
    // Minimum number of regions to add, exluding the required purple region.
    pub min_regions: usize,
    // Chance of adding an extra region after the minimum.
    pub extra_region_chance: f32,
    // Chance of adding an extra source to a region.
    pub extra_source_chance: f32,
    // Chance of adding any extra connection.
    pub extra_connection_chance: f32
}

impl Default for GeneratorSettings {
    fn default() -> Self {
        Self {
            size: GridSize::new(3, 3),
            swap_count: 4,
            missing_chance: 0.1,
            missing: 2,
            red_sources: SourceSettings::None,
            rotator_sources: SourceSettings::None,
            min_regions: 2,
            extra_region_chance: 0.1,
            extra_source_chance: 0.1,
            extra_connection_chance: 0.5,
        }
    }
}

pub fn generate_puzzle(settings: &GeneratorSettings) -> Puzzle {
    // let mut grid = generate_solution(settings);

    // println!("NEW PUZZLE");
    // let solution = reverse_solution(&mut grid, settings.swap_count);
    // println!("EXPECTED {solution:?}");

    // let puzzle = create_puzzle_from_grid(grid, solution.len() as u8);

    // if let Some(short_sol) = find_solution(&puzzle, solution.len() as u8 - 1) {
    //     println!("ACTUAL {short_sol:?}");
    // }

    let mut grid = Grid::with_size(GridSize::new(3, 3));
    grid.insert(GridIndex::new(0, 0), PuzzleCell::Normal { connections: DirectionSet::new(DirectionMapData { e: true, n: true, ..Default::default() }) });
    grid.insert(GridIndex::new(2, 2), PuzzleCell::Source { connections: DirectionSet::new(DirectionMapData { w: true, n: true, ..Default::default() }), source: Color::Purple });
    grid.insert(GridIndex::new(0, 2), PuzzleCell::Intersection { connections: DirectionMap::new(DirectionMapData { e: LayerConnection::Layer0, n: LayerConnection::Layer1, w: LayerConnection::Layer1, s: LayerConnection::Layer1 }) });
    grid.insert(GridIndex::new(1, 2), PuzzleCell::Intersection { connections: DirectionMap::new(DirectionMapData { e: LayerConnection::Layer0, n: LayerConnection::Layer1, w: LayerConnection::Layer0, s: LayerConnection::Layer1 }) });
    grid.insert(GridIndex::new(1, 1), PuzzleCell::Intersection { connections: DirectionMap::new(DirectionMapData { e: LayerConnection::Layer0, n: LayerConnection::Layer0, w: LayerConnection::Layer1, s: LayerConnection::Layer1 }) });
    let puzzle = Puzzle::new(grid, 4);

    puzzle
}

// fn create_puzzle_from_grid(game_grid: GameGrid, swaps: u8) -> Puzzle {
//     let mut grid = Grid::new();
//     for (pos, cell) in game_grid.iter() {nuy]
//         grid.insert(*pos, PuzzleCell::new(cell.connections(), cell.source()));
//     }
//     Puzzle::new(grid, swaps)
// }

// fn generate_solution(settings: &GeneratorSettings) -> GameGrid {
//     loop {
//         if let Some(solution) = generate_solution_maybe(settings) {
//             break solution
//         }
//     }
// }

// fn generate_solution_maybe(settings: &GeneratorSettings) -> Option<GameGrid> {
//     let mut grid = Grid::new();

//     fn populate_cells(grid: &mut Grid<PuzzleCell>, settings: &GeneratorSettings) {
//         let mut missing_count = 0;
//         for x in 0..settings.size.x {
//             for y in 0..settings.size.y {
//                 let missing = if missing_count >= settings.missing { false } else {
//                     rand::random::<f32>() < settings.missing_chance
//                 };
//                 if !missing {
//                     grid.insert(Pos2 { x, y }, PuzzleCell::nonsource(DirSet::new()));
//                 }
//                 else {
//                     missing_count += 1;
//                 }
//             }
//         }
//     }
//     populate_cells(&mut grid, settings);

//     fn add_sources(grid: &mut Grid<PuzzleCell>, settings: &GeneratorSettings) {
//         let mut possible: Vec<_> = grid.iter_mut().map(|(p, c)| (*p, c)).collect();
//         fn add_source(possible: &mut Vec<(Pos2, &mut PuzzleCell)>, color: Color) {
//             let index = rand::random::<usize>() % possible.len();
//             let selection = possible.swap_remove(index);
//             selection.1.source = Some(color);
//         }
//         add_source(&mut possible, Color::Purple);
//         let mut added = 0;
//         let mut possible_colors: Vec<Color> = vec![Color::Blue, Color::Green];
//         match settings.red_sources {
//             SourceSettings::None => (),
//             SourceSettings::Maybe => {
//                 possible_colors.push(Color::Red);
//             },
//             SourceSettings::Definitely => {
//                 added += 1;
//                 add_source(&mut possible, Color::Red);
//             },
//         }
//         match settings.rotator_sources {
//             SourceSettings::None => (),
//             SourceSettings::Maybe => {
//                 possible_colors.extend([Color::Orange, Color::Yellow]);
//             },
//             SourceSettings::Definitely => {
//                 added += 1;
//                 if rand::random::<bool>() {
//                     add_source(&mut possible, Color::Orange);
//                     possible_colors.push(Color::Yellow);
//                 }
//                 else {
//                     add_source(&mut possible, Color::Yellow);
//                     possible_colors.push(Color::Orange);
//                 }
//             }
//         }

//         while possible_colors.len() > 0 && (added < settings.min_regions || rand::random::<f32>() < settings.extra_region_chance) {
//             let color = possible_colors.swap_remove(rand::random::<usize>() % possible_colors.len());
//             add_source(&mut possible, color);
//             added += 1;
//         }
//     }
//     add_sources(&mut grid, settings);

//     fn connect_regions(grid: &mut Grid<PuzzleCell>, settings: &GeneratorSettings) {
//         #[derive(Debug)]
//         struct NeighborData {
//             pub from: Pos2,
//             pub to: Pos2,
//             pub dir: Dir,
//         }
//         struct RegionData<'a> {
//             pub positions: Vec<(Pos2, &'a mut PuzzleCell)>,
//             pub neighbors: Vec<NeighborData>
//         }

//         let mut regions = HashMap::new();
//         let mut to_fill: HashMap<Pos2, &mut PuzzleCell> = HashMap::new();
//         for (pos, cell) in grid.iter_mut() {
//             if let Some(source) = cell.source {
//                 cell.source = None;
//                 regions.insert(source, RegionData {
//                     positions: vec![(*pos, cell)],
//                     neighbors: Dir::ALL.into_iter().map(|dir| NeighborData {
//                         from: *pos,
//                         to: *pos + dir.to_vec(),
//                         dir
//                     }).collect()
//                 });
//             }
//             else {
//                 to_fill.insert(*pos, cell);
//             }
//         }
    
//         let mut colors: Vec<Color> = regions.keys().map(|c| *c).collect();
//         let mut priority_extend: Vec<Color> = colors.iter().filter(|c| match c {
//             Color::Red => true,
//             Color::Orange => true,
//             Color::Yellow => true,
//             Color::Green => false,
//             Color::Blue => false,
//             Color::Purple => true,
//         }).copied().collect();
//         priority_extend.shuffle(&mut rand::thread_rng());
//         while to_fill.len() > 0 {
//             if let Some(color) = priority_extend.pop().or_else(|| colors.choose(&mut rand::thread_rng()).copied()) {
//                 let region = regions.get_mut(&color).unwrap();
//                 let to_add = loop {
//                     if region.neighbors.len() == 0 {
//                         break None;
//                     }
//                     let neighbor = region.neighbors.swap_remove(rand::random::<usize>() % region.neighbors.len());
//                     if let Some(cell) = to_fill.remove(&neighbor.to) {
//                         break Some((cell, neighbor))
//                     }
//                 };
//                 if let Some((cell, to_add)) = to_add {
//                     cell.connections.insert(to_add.dir.inverse());
//                     region.positions.iter_mut().find(|p| p.0 == to_add.from).unwrap().1.connections.insert(to_add.dir);
//                     region.positions.push((to_add.to, cell));
//                     region.neighbors.extend(Dir::ALL.into_iter().map(|dir| NeighborData {
//                         from: to_add.to,
//                         to: to_add.to + dir.to_vec(),
//                         dir
//                     }));
//                 }
//                 else {
//                     if let Some(index) = colors.iter().enumerate()
//                         .find(|(_idx, at_color)| **at_color == color).map(|x| x.0) {
//                         colors.swap_remove(index);
//                     }
//                 }
//             }
//             else {
//                 break
//             }
//         }

//         fn add_extra_connections_in_regions(regions: &mut HashMap<Color, RegionData<'_>>, settings: &GeneratorSettings) {
//             for (_color, region) in regions.iter_mut() {
//                 if region.positions.len() <= 3 { continue; /* impossible to have extra pairs */}
//                 for i in 0..(region.positions.len() - 1) {
//                     for j in (i+1)..region.positions.len() {
//                         let pos_i = region.positions[i].0;
//                         let pos_j = region.positions[j].0;
//                         if let Some(dir) = pos_i.adjacent_dir(pos_j) {
//                             if !region.positions[i].1.connections.contains(dir) {
//                                 if rand::random::<f32>() < settings.extra_connection_chance {
//                                     region.positions[i].1.connections.insert(dir);
//                                     region.positions[j].1.connections.insert(dir.inverse());
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         add_extra_connections_in_regions(&mut regions, settings);

//         fn set_sources_in_regions(regions: HashMap<Color, RegionData<'_>>, settings: &GeneratorSettings) {
//             for (color, region) in regions {
//                 let mut positions = region.positions;
//                 let mut added = 0;
//                 while added < 1 || (positions.len() > 0 && rand::random::<f32>() < settings.extra_source_chance) {
//                     let entry = positions.swap_remove(rand::random::<usize>() % positions.len());
//                     added += 1;
//                     entry.1.source = Some(color);
//                 }
//             }
//         }
//         set_sources_in_regions(regions, settings);
//     }
//     connect_regions(&mut grid, settings);

//     fn remove_unused(grid: &mut Grid<PuzzleCell>) {
//         let to_remove: Vec<_> = grid.iter()
//             .filter(|(_p, cell)| (cell.source.is_none() || cell.source == Some(Color::Red)) && cell.connections.len() == 0)
//             .map(|(p, _)| *p)
//             .collect();
//         for pos in to_remove {
//             grid.remove(pos);
//         }
//         grid.recompute_bounds();
//     }
//     remove_unused(&mut grid);

//     fn is_ok(grid: &Grid<PuzzleCell>) -> bool {
//         grid.iter().all(|(_p, cell)| {
//             cell.connections.len() != 0 || (cell.source != Some(Color::Orange) && cell.source != Some(Color::Yellow))
//         })
//     }

//     if is_ok(&grid) {
//         Some(GameGrid::from_puzzle_grid(grid))
//     }
//     else {
//         None
//     }
// }

// fn swap_record_matches(grid: &GameGrid, record: SwapRecord) -> bool {
//     let cell_a = grid.get(record.a).unwrap();
//     let cell_b = grid.get(record.b).unwrap();
//     Cell::can_swap(cell_a, cell_b)
//         && cell_a.fill().get_rotation() == record.a_rotation
//         && cell_b.fill().get_rotation() == record.b_rotation
// }

// fn reverse_solution(grid: &mut GameGrid, swaps: u8) -> Vec<SwapRecord> {
//     let mut solution = Vec::new();
//     for _ in 0..swaps {
//         solution.push(reverse_swap(grid));
//     }
//     solution.reverse();
//     solution
// }

// fn reverse_swap(grid: &mut GameGrid) -> SwapRecord {
//     let mut possible_rotations = vec![Rotation::None];
//     if grid.iter().any(|(_, c)| c.fill().contains(Color::CCW)) {
//         possible_rotations.push(Rotation::CCW);
//     }
//     if grid.iter().any(|(_, c)| c.fill().contains(Color::CW)) {
//         possible_rotations.push(Rotation::CW);
//     }
//     let cells: Vec<_> = grid.iter().filter(|(_, c)| c.source() != Some(Color::Red))
//         .map(|(p, c)| (*p, *c)).collect();
//     let mut selected: [(Pos2, Cell); 2] = [(Pos2::ZERO, cells[0].1), (Pos2::ZERO, cells[0].1)];
//     let mut rotations: [Rotation; 2] = [Rotation::None, Rotation::None];
//     loop {
//         cells.choose_multiple(&mut rand::thread_rng(), selected.len())
//             .zip(selected.iter_mut()).for_each(|(s, buf)| *buf = *s);
//         rotations.iter_mut().for_each(|buf| *buf = *possible_rotations.choose(&mut rand::thread_rng()).unwrap());

//         let record = SwapRecord::new(selected[0].0, selected[1].0, rotations[0], rotations[1]);
//         grid.swap(record.a, record.b_rotation.inverse(), selected[0].1, record.b, record.a_rotation.inverse(), selected[1].1);

//         if swap_record_matches(grid, record) {
//             break record
//         }
//         else {
//             grid.swap(record.a, Rotation::None, selected[1].1, record.b, Rotation::None, selected[0].1);
//         }
//     }
// }