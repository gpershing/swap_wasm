use crate::grids::{Direction, DirectionMap, DirectionSet, Grid, GridIndex, GridSize};

use super::{Color, LayerConnection, Puzzle, PuzzleCell};

fn dirs(e: bool, n: bool, w: bool, s: bool) -> DirectionSet {
    let mut set = DirectionSet::empty();
    if e {
        set.insert(Direction::E);
    }
    if n {
        set.insert(Direction::N);
    }
    if w {
        set.insert(Direction::W);
    }
    if s {
        set.insert(Direction::S);
    }
    set
}

fn ldirs(e: usize, n: usize, w: usize, s: usize) -> DirectionMap<LayerConnection> {
    let mut map = DirectionMap::new_with_repeat(LayerConnection::None);
    if e > 0 {
        map[Direction::E] = if e == 1 {
            LayerConnection::Layer0
        } else {
            LayerConnection::Layer1
        };
    }
    if n > 0 {
        map[Direction::N] = if n == 1 {
            LayerConnection::Layer0
        } else {
            LayerConnection::Layer1
        };
    }
    if w > 0 {
        map[Direction::W] = if w == 1 {
            LayerConnection::Layer0
        } else {
            LayerConnection::Layer1
        };
    }
    if s > 0 {
        map[Direction::S] = if s == 1 {
            LayerConnection::Layer0
        } else {
            LayerConnection::Layer1
        };
    }
    map
}

#[allow(dead_code)]
pub fn test_puzzle() -> Puzzle {
    let mut grid = Grid::with_size(GridSize {
        width: 1,
        height: 4,
    });

    grid.insert(
        GridIndex::new(0, 0),
        PuzzleCell::Normal {
            connections: dirs(false, true, false, false),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(0, 3),
        PuzzleCell::Source {
            connections: dirs(false, false, false, true),
            source: Color::Purple,
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(0, 1),
        PuzzleCell::Normal {
            connections: dirs(false, true, false, true),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(0, 2),
        PuzzleCell::Normal {
            connections: dirs(false, true, false, true),
        },
    )
    .unwrap();

    Puzzle::new(grid, 4, GridIndex { x: 0, y: 0 })
}

#[allow(dead_code)]
pub fn debug_puzzle() -> Puzzle {
    let mut grid = Grid::with_size(GridSize {
        width: 10,
        height: 6,
    });

    grid.insert(
        GridIndex::new(0, 0),
        PuzzleCell::Normal {
            connections: dirs(false, false, false, false),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(0, 1),
        PuzzleCell::Source {
            connections: dirs(false, false, false, false),
            source: Color::Purple,
        },
    )
    .unwrap();

    grid.insert(
        GridIndex::new(1, 0),
        PuzzleCell::Normal {
            connections: dirs(true, false, false, false),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(1, 1),
        PuzzleCell::Normal {
            connections: dirs(false, true, false, false),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(1, 2),
        PuzzleCell::Normal {
            connections: dirs(false, false, true, false),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(1, 3),
        PuzzleCell::Normal {
            connections: dirs(false, false, false, true),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(1, 4),
        PuzzleCell::Source {
            connections: dirs(true, false, false, false),
            source: Color::Blue,
        },
    )
    .unwrap();

    grid.insert(
        GridIndex::new(2, 0),
        PuzzleCell::Normal {
            connections: dirs(true, true, false, false),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(2, 1),
        PuzzleCell::Normal {
            connections: dirs(false, true, true, false),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(2, 2),
        PuzzleCell::Normal {
            connections: dirs(false, false, true, true),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(2, 3),
        PuzzleCell::Normal {
            connections: dirs(true, false, false, true),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(2, 4),
        PuzzleCell::Source {
            connections: dirs(true, true, false, false),
            source: Color::Green,
        },
    )
    .unwrap();

    grid.insert(
        GridIndex::new(3, 0),
        PuzzleCell::Normal {
            connections: dirs(true, false, true, false),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(3, 1),
        PuzzleCell::Normal {
            connections: dirs(false, true, false, true),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(3, 2),
        PuzzleCell::Source {
            connections: dirs(true, false, true, false),
            source: Color::Yellow,
        },
    )
    .unwrap();

    grid.insert(
        GridIndex::new(4, 0),
        PuzzleCell::Normal {
            connections: dirs(true, true, true, false),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(4, 1),
        PuzzleCell::Normal {
            connections: dirs(false, true, true, true),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(4, 2),
        PuzzleCell::Normal {
            connections: dirs(true, false, true, true),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(4, 3),
        PuzzleCell::Normal {
            connections: dirs(true, true, false, true),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(4, 4),
        PuzzleCell::Source {
            connections: dirs(true, true, true, false),
            source: Color::Orange,
        },
    )
    .unwrap();

    grid.insert(
        GridIndex::new(5, 0),
        PuzzleCell::Normal {
            connections: dirs(true, true, true, true),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(5, 5),
        PuzzleCell::Source {
            connections: dirs(true, true, true, true),
            source: Color::Red,
        },
    )
    .unwrap();

    grid.insert(
        GridIndex::new(7, 0),
        PuzzleCell::Intersection {
            connections: ldirs(1, 2, 2, 2),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(7, 1),
        PuzzleCell::Intersection {
            connections: ldirs(2, 1, 2, 2),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(7, 2),
        PuzzleCell::Intersection {
            connections: ldirs(2, 2, 1, 2),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(7, 3),
        PuzzleCell::Intersection {
            connections: ldirs(2, 2, 2, 1),
        },
    )
    .unwrap();

    grid.insert(
        GridIndex::new(8, 0),
        PuzzleCell::Intersection {
            connections: ldirs(1, 1, 2, 2),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(8, 1),
        PuzzleCell::Intersection {
            connections: ldirs(2, 1, 1, 2),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(8, 2),
        PuzzleCell::Intersection {
            connections: ldirs(1, 2, 2, 0),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(8, 3),
        PuzzleCell::Intersection {
            connections: ldirs(0, 1, 2, 2),
        },
    )
    .unwrap();

    grid.insert(
        GridIndex::new(9, 0),
        PuzzleCell::Intersection {
            connections: ldirs(1, 2, 1, 2),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(9, 1),
        PuzzleCell::Intersection {
            connections: ldirs(1, 2, 0, 2),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(9, 2),
        PuzzleCell::Intersection {
            connections: ldirs(2, 1, 2, 0),
        },
    )
    .unwrap();
    grid.insert(
        GridIndex::new(9, 4),
        PuzzleCell::Intersection {
            connections: ldirs(2, 2, 2, 2),
        },
    )
    .unwrap();

    Puzzle::new(grid, 4, GridIndex { x: 0, y: 0 })
}
