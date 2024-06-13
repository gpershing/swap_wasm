use std::f32::consts::TAU;

use egui::{epaint::CubicBezierShape, Color32, Painter, Pos2, Rect, Shape, Stroke, Vec2};

use crate::{gameplay::{Cell, CellLayer, Color}, grids::{Direction, GridIndex}};

use super::{palette::Palette, simulation::Simulation, SegmentMeshData};

pub struct CellDrawData<'a> {
    pub center: Pos2,
    pub index: GridIndex,
    pub size: f32,
    pub mesh_data: &'a SegmentMeshData,
    pub simulation: &'a Simulation,
    pub animation_t: f32,
    pub palette: &'a Palette
}

pub fn draw_cell(cell: &Cell, painter: &Painter, data: CellDrawData<'_>) {
    if cell.get_layer_count() == 1 {
        draw_simple(cell.get_layer(0).unwrap(), cell.source(), painter, data);
    }
    else {
        let layers: Vec<_> = cell.iter_layers().collect();
        if layers[0].connections.is_empty() {
            draw_simple(layers[1], None, painter, data);
        }
        else if layers[1].connections.is_empty() {
            draw_simple(layers[0], None, painter, data);
        }
        else {
            draw_intersection(layers[0], layers[1], painter, data);
        }
    }
}

fn octagon(center: Pos2, radius: f32) -> Vec<Pos2> {
    (0..8)
        .map(|i| (i as f32 + 0.5) * TAU / 8.0)
        .map(|theta| center + Vec2::angled(theta) * radius)
        .collect()
}

fn tri_rotated(center: Pos2, radius: f32, rotation: f32) -> Vec<Pos2> {
    (0..3)
        .map(|i| (i as f32) * TAU / 3.0 + rotation)
        .map(|theta| center + Vec2::angled(theta) * radius)
        .collect()
}

fn draw_swap_source(painter: &Painter, data: CellDrawData<'_>) {
    let closed = false;
    let fill = Color32::TRANSPARENT;
    let stroke = Stroke::new(0.01 * data.size, data.palette.get(Color::SWAP));
    painter.extend((0..6).map(|i| (i as f32 + 0.5) * TAU / 6.0)
        .flat_map(|theta| {
            let end = data.center + Vec2::angled(theta) * data.size * 0.09;
            let tan = Vec2::angled(theta + TAU * 0.25) * data.size * 0.025;
            [
                Shape::CubicBezier(CubicBezierShape { points: [
                    data.center,
                    data.center + tan,
                    end + tan,
                    end
                ], closed, fill, stroke }),
                Shape::CubicBezier(CubicBezierShape { points: [
                    end,
                    end - tan,
                    data.center - tan,
                    data.center
                ], closed, fill, stroke })
            ]
        }));
}

fn draw_source(source: Color, painter: &Painter, data: CellDrawData<'_>) {
    let rotator_speed: f32 = 1.0;
    match source {
        Color::STOP => { painter.add(Shape::convex_polygon(octagon(data.center, data.size * 0.08), data.palette.get(source), Stroke::new(0.0, Color32::TRANSPARENT))); },
        Color::CCW => { painter.add(Shape::convex_polygon(tri_rotated(data.center, data.size * 0.06, data.animation_t * -rotator_speed), Color32::TRANSPARENT, Stroke::new(data.size * 0.01, data.palette.get(source)))); },
        Color::CW => { painter.add(Shape::convex_polygon(tri_rotated(data.center, data.size * 0.06, data.animation_t * rotator_speed), Color32::TRANSPARENT, Stroke::new(data.size * 0.01, data.palette.get(source)))); },
        Color::Green => { painter.rect_stroke(Rect::from_center_size(data.center, Vec2::splat(data.size * 0.12)), 0.0, Stroke::new(data.size * 0.01, data.palette.get(source))); },
        Color::Blue => { painter.circle_stroke(data.center, data.size * 0.06, Stroke::new(data.size * 0.01, data.palette.get(source))); },
        Color::SWAP => draw_swap_source(painter, data)
    };
}

fn draw_simple(layer: &CellLayer, source: Option<Color>, painter: &Painter, data: CellDrawData<'_>) {
    let connections: Vec<_> = layer.connections.iter_set().collect();
    if connections.is_empty() {
        let color = source.map(|s| data.palette.get(s)).unwrap_or(data.palette.empty);
        for r in 0..8 {
            let rotation = (r as f32) * TAU * 0.125;
            let cos = rotation.cos();
            let sin = rotation.sin();
            painter.add(Shape::Mesh(data.mesh_data.o0.get_mesh(|point| Pos2 {
                x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                y: (point.x * -sin + point.y * cos) * data.size + data.center.y
            }, |_t| color)));
        }
        painter.circle_stroke(data.center, data.size * 0.3, Stroke::new(0.03 * data.size, color));
    }
    else if connections.len() == 1 {
        let rotation = match connections[0] {
            Direction::E => 0.0,
            Direction::N => TAU * 0.25,
            Direction::W => TAU * 0.50,
            Direction::S => TAU * 0.75,
        };
        let cos = rotation.cos();
        let sin = rotation.sin();
        let t_default = |point: Pos2| Pos2 {
            x: (point.x * cos + point.y * sin) * data.size + data.center.x,
            y: (point.x * -sin + point.y * cos) * data.size + data.center.y
        };
        let t_mirror = |point: Pos2| Pos2 {
            x: (point.x * cos - point.y * sin) * data.size + data.center.x,
            y: (point.x * -sin - point.y * cos) * data.size + data.center.y
        };
        painter.add(Shape::Mesh(data.mesh_data.c0.get_mesh(t_default, data.simulation.color_fn_single((data.index, connections[0])))));
        painter.add(Shape::Mesh(data.mesh_data.c0.get_mesh(t_mirror, data.simulation.color_fn_single((data.index, connections[0])))));
    }
    else if connections.len() == 2 {
        if connections[0] == connections[1].inverse() {
            let ew = connections[0] == Direction::E || connections[0] == Direction::W;
            let rotation = if ew { 0.0 } else { TAU * 0.25 };
            let cos = rotation.cos();
            let sin = rotation.sin();

            let t_default = |point: Pos2| Pos2 {
                x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                y: (point.x * -sin + point.y * cos) * data.size + data.center.y
            };
            let t_mirror = |point: Pos2| Pos2 {
                x: (point.x * cos - point.y * sin) * data.size + data.center.x,
                y: (point.x * -sin - point.y * cos) * data.size + data.center.y
            };
            let (dir1, dir2) = if ew { (Direction::E, Direction::W) } else { (Direction::N, Direction::S) };
            painter.add(Shape::Mesh(data.mesh_data.h0.get_mesh(t_default, data.simulation.color_fn_through_two((data.index, dir1), (data.index, dir2)))));
            painter.add(Shape::Mesh(data.mesh_data.h0.get_mesh(t_mirror, data.simulation.color_fn_through_two((data.index, dir1), (data.index, dir2)))));
        }
        else {
            let e = connections[0] == Direction::E || connections[1] == Direction::E;
            let n = connections[0] == Direction::N || connections[1] == Direction::N;
            let rotation = match (e, n) {
                (true, true) => TAU * 0.25,
                (true, false) => 0.0,
                (false, true) => TAU * 0.5,
                (false, false) => TAU * 0.75,
            };
            let (dir2, dir1) = match (e, n) {
                (true, true) => (Direction::E, Direction::N),
                (true, false) => (Direction::S, Direction::E),
                (false, true) => (Direction::N, Direction::W),
                (false, false) => (Direction::W, Direction::S),
            };
            let cos = rotation.cos();
            let sin = rotation.sin();
            let t_default = |point: Pos2| Pos2 {
                x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                y: (point.x * -sin + point.y * cos) * data.size + data.center.y
            };
            painter.add(Shape::Mesh(data.mesh_data.l0.get_mesh(t_default, data.simulation.color_fn_through_two((data.index, dir1), (data.index, dir2)))));
            painter.add(Shape::Mesh(data.mesh_data.l1.get_mesh(t_default, data.simulation.color_fn_through_two((data.index, dir1), (data.index, dir2)))));
        }
    }
    else if connections.len() == 3 {
        let missing = Direction::ALL.into_iter().find(|dir| !connections.contains(dir)).unwrap();
        let rotation = match missing {
            Direction::N => 0.0,
            Direction::W => TAU * 0.25,
            Direction::S => TAU * 0.50,
            Direction::E => TAU * 0.75,
        };
        let (dir1, dir2, dir3) = match missing {
            Direction::E => (Direction::N, Direction::W, Direction::S),
            Direction::N => (Direction::W, Direction::S, Direction::E),
            Direction::W => (Direction::S, Direction::E, Direction::N),
            Direction::S => (Direction::E, Direction::N, Direction::W)
        };
        let cos = rotation.cos();
        let sin = rotation.sin();
        let t_default = |point: Pos2| Pos2 {
            x: (point.x * cos + point.y * sin) * data.size + data.center.x,
            y: (point.x * -sin + point.y * cos) * data.size + data.center.y
        };
        let t_rotate = |point: Pos2| Pos2 {
            x: (-point.x * cos + point.y * sin) * data.size + data.center.x,
            y: (-point.x * -sin + point.y * cos) * data.size + data.center.y
        };
        painter.add(Shape::Mesh(data.mesh_data.l0.get_mesh(t_default, data.simulation.color_fn_through_two((data.index, dir3), (data.index, dir2)) )));
        painter.add(Shape::Mesh(data.mesh_data.l0.get_mesh(t_rotate, data.simulation.color_fn_through_two((data.index, dir1), (data.index, dir2)) )));
        painter.add(Shape::Mesh(data.mesh_data.t0.get_mesh(t_default, data.simulation.color_fn_through_two((data.index, dir3), (data.index, dir1)) )));
    }
    else {
        let rotation: f32 = 0.0;
        let mut cos = rotation.cos();
        let mut sin = rotation.sin();
        for direction in Direction::ALL {
            let transform = |point: Pos2| Pos2 {
                x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                y: (point.x * -sin + point.y * cos) * data.size + data.center.y
            };
            painter.add(Shape::Mesh(data.mesh_data.l0.get_mesh(transform, data.simulation.color_fn_through_two((data.index, direction), (data.index, direction.rotated(crate::grids::Rotation::Clockwise))) )));
            std::mem::swap(&mut cos, &mut sin);
            cos = -cos;
        }
    }
    if let Some(source) = source {
        draw_source(source, painter, data);
    }
}

pub fn draw_intersection(layer0: &CellLayer, layer1: &CellLayer, painter: &Painter, data: CellDrawData<'_>) {
    draw_intersection_layer(layer0, layer1, true, painter, &data);
    draw_intersection_layer(layer1, layer0, false, painter, &data);
}

pub fn draw_intersection_layer(layer: &CellLayer, other_layer: &CellLayer, is_layer_zero: bool, painter: &Painter, data: &CellDrawData<'_>) {
    let connections: Vec<_> = layer.connections.iter_set().collect();
    if connections.len() == 1 {
        let rotation = match connections[0] {
            Direction::E => 0.0,
            Direction::N => TAU * 0.25,
            Direction::W => TAU * 0.50,
            Direction::S => TAU * 0.75,
        };
        let cos = rotation.cos();
        let sin = rotation.sin();
        painter.add(Shape::Mesh(data.mesh_data.ic0.get_mesh(|point| Pos2 {
            x: (point.x * cos + point.y * sin) * data.size + data.center.x,
            y: (point.x * -sin + point.y * cos) * data.size + data.center.y
        }, data.simulation.color_fn_single_both_ways((data.index, connections[0])))));
    }
    else if connections.len() == 2 {
        if connections[0] == connections[1].inverse() {
            let ew = connections[0] == Direction::E || connections[0] == Direction::W;
            let (dir1, dir2) = if ew { (Direction::E, Direction::W) } else { (Direction::N, Direction::S) };
            if other_layer.connections.len() == 2 {
                if is_layer_zero {
                    let rotation = if ew { 0.0 } else { TAU * 0.25 };
                    let cos = rotation.cos();
                    let sin = rotation.sin();
                    painter.add(Shape::Mesh(data.mesh_data.ih0.get_mesh(|point| Pos2 {
                        x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                        y: (point.x * -sin + point.y * cos) * data.size + data.center.y
                    }, data.simulation.color_fn_through_two((data.index, dir1), (data.index, dir2)))));
                }
                else {
                    let rotation = if ew { 0.0 } else { TAU * 0.25 };
                    let cos = rotation.cos();
                    let sin = rotation.sin();
                    painter.add(Shape::Mesh(data.mesh_data.ih1.get_mesh(|point| Pos2 {
                        x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                        y: (point.x * -sin + point.y * cos) * data.size + data.center.y
                    }, data.simulation.color_fn_through_two((data.index, dir1), (data.index, dir2)))));
                }
            }
            else {
                let (rotation, dir1) = if ew {
                    if other_layer.connections.contains(Direction::N) { (TAU * 0.5, Direction::W) }
                    else { (0.0, Direction::E) }
                }
                else if other_layer.connections.contains(Direction::W) { (TAU * 0.75, Direction::S) }
                else { (TAU * 0.25, Direction::N) };
                let dir2 = dir1.inverse();
                let cos = rotation.cos();
                let sin = rotation.sin();
                painter.add(Shape::Mesh(data.mesh_data.h0.get_mesh(|point| Pos2 {
                    x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                    y: (point.x * -sin + point.y * cos) * data.size + data.center.y
                }, data.simulation.color_fn_through_two((data.index, dir1), (data.index, dir2)))));
            }
        }
        else {
            let e = connections[0] == Direction::E || connections[1] == Direction::E;
            let n = connections[0] == Direction::N || connections[1] == Direction::N;
            let rotation = match (e, n) {
                (true, true) => TAU * 0.25,
                (true, false) => 0.0,
                (false, true) => TAU * 0.5,
                (false, false) => TAU * 0.75,
            };
            let (dir2, dir1) = match (e, n) {
                (true, true) => (Direction::E, Direction::N),
                (true, false) => (Direction::S, Direction::E),
                (false, true) => (Direction::N, Direction::W),
                (false, false) => (Direction::W, Direction::S),
            };
            let cos = rotation.cos();
            let sin = rotation.sin();
            painter.add(Shape::Mesh(data.mesh_data.il0.get_mesh(|point| Pos2 {
                x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                y: (point.x * -sin + point.y * cos) * data.size + data.center.y
            }, data.simulation.color_fn_through_two((data.index, dir1), (data.index, dir2)))));
        }
    }    
    else if connections.len() == 3 {
        let missing = Direction::ALL.into_iter().find(|dir| !connections.contains(dir)).unwrap();
        let rotation = match missing {
            Direction::N => 0.0,
            Direction::W => TAU * 0.25,
            Direction::S => TAU * 0.50,
            Direction::E => TAU * 0.75,
        };
        let (dir1, dir2, dir3) = match missing {
            Direction::E => (Direction::N, Direction::W, Direction::S),
            Direction::N => (Direction::W, Direction::S, Direction::E),
            Direction::W => (Direction::S, Direction::E, Direction::N),
            Direction::S => (Direction::E, Direction::N, Direction::W)
        };
        let mut cos = rotation.cos();
        let mut sin = rotation.sin();
        painter.add(Shape::Mesh(data.mesh_data.l0.get_mesh(|point| Pos2 {
            x: (point.x * cos + point.y * sin) * data.size + data.center.x,
            y: (point.x * -sin + point.y * cos) * data.size + data.center.y
        },  data.simulation.color_fn_through_two((data.index, dir3), (data.index, dir2)))));
        std::mem::swap(&mut cos, &mut sin);
        sin = -sin;
        painter.add(Shape::Mesh(data.mesh_data.l0.get_mesh(|point| Pos2 {
            x: (point.x * cos + point.y * sin) * data.size + data.center.x,
            y: (point.x * -sin + point.y * cos) * data.size + data.center.y
        }, data.simulation.color_fn_through_two((data.index, dir2), (data.index, dir1)))));
    }
}