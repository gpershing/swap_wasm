use std::f32::consts::TAU;

use eframe::egui_glow::painter;
use egui::{accesskit::Affine, epaint::CubicBezierShape, Color32, Mesh, Painter, Pos2, Shape, Stroke, Vec2};

use crate::{gameplay::{Cell, CellLayer, Color}, grids::Direction};

use super::{bezier::{CubicBezierControl, CubicBezierMesh, CubicBezierPoint}, segment::{Segment, SEGMENT_C0}, SegmentMeshData};

pub struct CellDrawData<'a> {
    pub center: Pos2,
    pub size: f32,
    pub mesh_data: &'a SegmentMeshData
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
            draw_simple(layers[1], None, painter, data);
        }
        else {
            draw_intersection(layers[0], layers[1], painter, data);
        }
    }
}

fn draw_simple(layer: &CellLayer, source: Option<Color>, painter: &Painter, data: CellDrawData<'_>) {
    let connections: Vec<_> = layer.connections.iter_set().collect();
    if connections.len() == 0 {
        let color = source.map(|s| s.color32()).unwrap_or(Color32::WHITE);
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
        painter.add(Shape::Mesh(data.mesh_data.c0.get_mesh(|point| Pos2 {
            x: (point.x * cos + point.y * sin) * data.size + data.center.x,
            y: (point.x * -sin + point.y * cos) * data.size + data.center.y
        }, |t| {
            Color32::from_rgb((t * 255.0) as u8, 0, 0)
        })));
        painter.add(Shape::Mesh(data.mesh_data.c0.get_mesh(|point| Pos2 {
            x: (point.x * cos - point.y * sin) * data.size + data.center.x,
            y: (point.x * -sin - point.y * cos) * data.size + data.center.y
        }, |t| {
            Color32::from_rgb((t * 255.0) as u8, 0, 0)
        })));
    }
    else if connections.len() == 2 {
        if connections[0] == connections[1].inverse() {
            let ew = connections[0] == Direction::E || connections[0] == Direction::W;
            let rotation = if ew { 0.0 } else { TAU * 0.25 };
            let cos = rotation.cos();
            let sin = rotation.sin();
            painter.add(Shape::Mesh(data.mesh_data.h0.get_mesh(|point| Pos2 {
                x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                y: (point.x * -sin + point.y * cos) * data.size + data.center.y
            }, |t| {
                Color32::from_rgb((t * 255.0) as u8, 0, 0)
            })));
            painter.add(Shape::Mesh(data.mesh_data.h0.get_mesh(|point| Pos2 {
                x: (point.x * cos - point.y * sin) * data.size + data.center.x,
                y: (point.x * -sin - point.y * cos) * data.size + data.center.y
            }, |t| {
                Color32::from_rgb((t * 255.0) as u8, 0, 0)
            })));
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
            let cos = rotation.cos();
            let sin = rotation.sin();
            painter.add(Shape::Mesh(data.mesh_data.l0.get_mesh(|point| Pos2 {
                x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                y: (point.x * -sin + point.y * cos) * data.size + data.center.y
            }, |t| {
                Color32::from_rgb((t * 255.0) as u8, 0, 0)
            })));
            painter.add(Shape::Mesh(data.mesh_data.l1.get_mesh(|point| Pos2 {
                x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                y: (point.x * -sin + point.y * cos) * data.size + data.center.y
            }, |t| {
                Color32::from_rgb((t * 255.0) as u8, 0, 0)
            })));
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
        let cos = rotation.cos();
        let sin = rotation.sin();
        painter.add(Shape::Mesh(data.mesh_data.l0.get_mesh(|point| Pos2 {
            x: (point.x * cos + point.y * sin) * data.size + data.center.x,
            y: (point.x * -sin + point.y * cos) * data.size + data.center.y
        }, |t| {
            Color32::from_rgb((t * 255.0) as u8, 0, 0)
        })));
        painter.add(Shape::Mesh(data.mesh_data.l0.get_mesh(|point| Pos2 {
            x: (-point.x * cos + point.y * sin) * data.size + data.center.x,
            y: (-point.x * -sin + point.y * cos) * data.size + data.center.y
        }, |t| {
            Color32::from_rgb((t * 255.0) as u8, 0, 0)
        })));
        painter.add(Shape::Mesh(data.mesh_data.t0.get_mesh(|point| Pos2 {
            x: (point.x * cos + point.y * sin) * data.size + data.center.x,
            y: (point.x * -sin + point.y * cos) * data.size + data.center.y
        }, |t| {
            Color32::from_rgb((t * 255.0) as u8, 0, 0)
        })));
    }
    else {
        let rotation: f32 = 0.0;
        let mut cos = rotation.cos();
        let mut sin = rotation.sin();
        for _ in 0..4 {
            painter.add(Shape::Mesh(data.mesh_data.l0.get_mesh(|point| Pos2 {
                x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                y: (point.x * -sin + point.y * cos) * data.size + data.center.y
            }, |t| {
                Color32::from_rgb((t * 255.0) as u8, 0, 0)
            })));
            std::mem::swap(&mut cos, &mut sin);
            sin = -sin;
        }
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
        }, |t| {
            Color32::from_rgb((t * 255.0) as u8, 0, 0)
        })));
    }
    else if connections.len() == 2 {
        if connections[0] == connections[1].inverse() {
            let ew = connections[0] == Direction::E || connections[0] == Direction::W;
            if other_layer.connections.len() == 2 {
                if is_layer_zero {
                    let rotation = if ew { 0.0 } else { TAU * 0.25 };
                    let cos = rotation.cos();
                    let sin = rotation.sin();
                    painter.add(Shape::Mesh(data.mesh_data.ih0.get_mesh(|point| Pos2 {
                        x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                        y: (point.x * -sin + point.y * cos) * data.size + data.center.y
                    }, |t| {
                        Color32::from_rgb((t * 255.0) as u8, 0, 0)
                    })));
                }
                else {
                    let rotation = if ew { 0.0 } else { TAU * 0.25 };
                    let cos = rotation.cos();
                    let sin = rotation.sin();
                    painter.add(Shape::Mesh(data.mesh_data.ih1.get_mesh(|point| Pos2 {
                        x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                        y: (point.x * -sin + point.y * cos) * data.size + data.center.y
                    }, |t| {
                        Color32::from_rgb((t * 255.0) as u8, 0, 0)
                    })));
                }
            }
            else {
                let rotation = if ew {
                    if other_layer.connections.contains(Direction::N) { TAU * 0.5 }
                    else { 0.0 }
                }
                else {
                    if other_layer.connections.contains(Direction::W) { TAU * 0.75 }
                    else { TAU * 0.25 }
                };
                let cos = rotation.cos();
                let sin = rotation.sin();
                painter.add(Shape::Mesh(data.mesh_data.h0.get_mesh(|point| Pos2 {
                    x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                    y: (point.x * -sin + point.y * cos) * data.size + data.center.y
                }, |t| {
                    Color32::from_rgb((t * 255.0) as u8, 0, 0)
                })));
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
            let cos = rotation.cos();
            let sin = rotation.sin();
            painter.add(Shape::Mesh(data.mesh_data.il0.get_mesh(|point| Pos2 {
                x: (point.x * cos + point.y * sin) * data.size + data.center.x,
                y: (point.x * -sin + point.y * cos) * data.size + data.center.y
            }, |t| {
                Color32::from_rgb((t * 255.0) as u8, 0, 0)
            })));
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
        let mut cos = rotation.cos();
        let mut sin = rotation.sin();
        painter.add(Shape::Mesh(data.mesh_data.l0.get_mesh(|point| Pos2 {
            x: (point.x * cos + point.y * sin) * data.size + data.center.x,
            y: (point.x * -sin + point.y * cos) * data.size + data.center.y
        }, |t| {
            Color32::from_rgb((t * 255.0) as u8, 0, 0)
        })));
        std::mem::swap(&mut cos, &mut sin);
        sin = -sin;
        painter.add(Shape::Mesh(data.mesh_data.l0.get_mesh(|point| Pos2 {
            x: (point.x * cos + point.y * sin) * data.size + data.center.x,
            y: (point.x * -sin + point.y * cos) * data.size + data.center.y
        }, |t| {
            Color32::from_rgb((t * 255.0) as u8, 0, 0)
        })));
    }
}