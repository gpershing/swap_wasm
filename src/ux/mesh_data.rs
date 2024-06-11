use egui::{Pos2, Vec2};

use super::bezier::{CubicBezierControl, CubicBezierMesh, CubicBezierPoint};

pub struct SegmentMeshData {
    pub c0: CubicBezierMesh,
    pub l0: CubicBezierMesh,
    pub l1: CubicBezierMesh,
    pub h0: CubicBezierMesh,
    pub t0: CubicBezierMesh,
    pub o0: CubicBezierMesh,

    pub ic0: CubicBezierMesh,
    pub ih0: CubicBezierMesh,
    pub ih1: CubicBezierMesh,
    pub il0: CubicBezierMesh
}

macro_rules! p {
    ($x:literal, $y: literal) => {
        Pos2 { x: $x, y: $y }
    };
}

macro_rules! v {
    ($x:literal, $y: literal) => {
        Vec2 { x: $x, y: $y }
    };
}
const fn mirrored(point: Pos2, handle1: Vec2) -> CubicBezierPoint {
    CubicBezierPoint { point, handles: CubicBezierControl::Mirrored(handle1) }
}

const fn free(point: Pos2, handle1: Vec2, handle2: Vec2) -> CubicBezierPoint {
    CubicBezierPoint { point, handles: CubicBezierControl::Free(handle1, handle2) }
}

impl SegmentMeshData {
    pub fn init(width: f32, feathering: f32, precision: f32) -> Self {
        Self {
            c0: CubicBezierMesh::new(vec![
                mirrored(p!(0.5, 0.0), v!(-0.01, 0.0)),
                free(p!(0.47, 0.0), v!(0.01, 0.0), v!(-0.05, 0.0)),
                free(p!(0.34, -0.05), v!(0.07, 0.0), v!(-0.12, 0.0)),
                free(p!(0.0, 0.14), v!(0.12, 0.0), v!(-0.077, 0.0)),
                mirrored(p!(-0.14, 0.0), v!(0.0, -0.077))
            ].as_slice(), width, feathering, precision),

            l0: CubicBezierMesh::new(vec![
                mirrored(p!(0.5, 0.0), v!(-0.01, 0.0)),
                free(p!(0.47, 0.0), v!(0.01, 0.0), v!(-0.05, 0.0)),
                mirrored(p!(0.34, -0.05), v!(-0.07, 0.0)),
                mirrored(p!(0.1, 0.1), v!(-0.05, 0.05)),
                mirrored(p!(-0.05, 0.34), v!(0.0, 0.07)),
                free(p!(0.0, 0.47), v!(0.0, -0.05), v!(0.0, 0.01)),
                mirrored(p!(0.0, 0.5), v!(0.0, 0.01))
            ].as_slice(), width, feathering, precision),

            l1: CubicBezierMesh::new(vec![
                mirrored(p!(0.5, 0.0), v!(-0.01, 0.0)),
                free(p!(0.47, 0.0), v!(0.01, 0.0), v!(-0.05, 0.0)),
                mirrored(p!(0.34, 0.05), v!(-0.07, 0.0)),
                mirrored(p!(0.1, -0.1), v!(-0.055, -0.055)),
                mirrored(p!(-0.1, -0.1), v!(-0.055, 0.055)),
                mirrored(p!(-0.1, 0.1), v!(0.055, 0.055)),
                mirrored(p!(0.05, 0.34), v!(0.0, 0.07)),
                free(p!(0.0, 0.47), v!(0.0, -0.05), v!(0.0, 0.01)),
                mirrored(p!(0.0, 0.5), v!(0.0, 0.01))
            ].as_slice(), width, feathering, precision),

            h0: CubicBezierMesh::new(vec![
                mirrored(p!(0.5, 0.0), v!(-0.01, 0.0)),
                free(p!(0.47, 0.0), v!(0.01, 0.0), v!(-0.05, 0.0)),
                free(p!(0.34, -0.05), v!(0.07, 0.0), v!(-0.06, 0.0)),
                mirrored(p!(0.23, 0.05), v!(-0.06, 0.0)),
                mirrored(p!(0.0, -0.14), v!(-0.15, 0.0)),
                mirrored(p!(-0.23, 0.05), v!(-0.06, 0.0)),
                free(p!(-0.34, -0.05), v!(0.07, 0.0), v!(-0.06, 0.0)),
                free(p!(-0.47, 0.0), v!(0.01, 0.0), v!(-0.05, 0.0)),
                mirrored(p!(-0.5, 0.0), v!(-0.01, 0.0))
            ].as_slice(), width, feathering, precision),

            t0: CubicBezierMesh::new(vec![
                mirrored(p!(0.5, 0.0), v!(-0.01, 0.0)),
                free(p!(0.47, 0.0), v!(0.01, 0.0), v!(-0.05, 0.0)),
                free(p!(0.34, 0.05), v!(0.07, 0.0), v!(-0.13, 0.0)),
                mirrored(p!(0.0, -0.14), v!(-0.14, 0.0)),
                free(p!(-0.34, 0.05), v!(0.13, 0.0), v!(-0.07, 0.0)),
                free(p!(-0.47, 0.0), v!(0.01, 0.0), v!(-0.05, 0.0)),
                mirrored(p!(-0.5, 0.0), v!(-0.01, 0.0))
            ].as_slice(), width, feathering, precision),

            o0: CubicBezierMesh::new(vec![
                mirrored(p!(0.3, 0.0), v!(-0.04, -0.03)),
                mirrored(p!(0.12, 0.0497), v!(-0.0207, 0.05)),
                mirrored(p!(0.212, 0.212), v!(0.0495, 0.0071))
            ].as_slice(), width, feathering, precision),
            
            ic0: CubicBezierMesh::new(vec![
                mirrored(p!(0.5, 0.0), v!(-0.01, 0.0)),
                free(p!(0.47, 0.0), v!(0.01, 0.0), v!(-0.05, 0.0)),
                mirrored(p!(0.31, -0.09), v!(-0.07, 0.0)),
                mirrored(p!(0.22, 0.0), v!(0.0, 0.030)),
                mirrored(p!(0.31, 0.09), v!(0.07, 0.0)),
                free(p!(0.47, 0.0), v!(-0.05, 0.0), v!(0.01, 0.0)),
                mirrored(p!(0.5, 0.0), v!(0.01, 0.0)),
            ].as_slice(), width, feathering, precision),

            ih0: CubicBezierMesh::new(vec![
                mirrored(p!(0.5, 0.0), v!(-0.01, 0.0)),
                mirrored(p!(-0.5, 0.0), v!(-0.01, 0.0))
            ].as_slice(), width, feathering, precision),

            ih1: CubicBezierMesh::new(vec![
                mirrored(p!(0.5, 0.0), v!(-0.01, 0.0)),
                free(p!(0.47, 0.0), v!(0.01, 0.0), v!(-0.05, 0.0)),
                free(p!(0.34, -0.05), v!(0.07, 0.0), v!(-0.06, 0.0)),
                mirrored(p!(0.07, 0.05), v!(-0.06, 0.0)),
                mirrored(p!(-0.07, -0.05), v!(-0.06, 0.0)),
                free(p!(-0.34, 0.05), v!(0.07, 0.0), v!(-0.06, 0.0)),
                free(p!(-0.47, 0.0), v!(0.01, 0.0), v!(-0.05, 0.0)),
                mirrored(p!(-0.5, 0.0), v!(-0.01, 0.0))
            ].as_slice(), width, feathering, precision),

            il0: CubicBezierMesh::new(vec![
                mirrored(p!(0.5, 0.0), v!(-0.01, 0.0)),
                free(p!(0.47, 0.0), v!(0.01, 0.0), v!(-0.05, 0.0)),
                free(p!(0.34, 0.05), v!(0.07, 0.0), v!(-0.13, 0.0)),
                free(p!(0.05, 0.34), v!(0.0, -0.13), v!(0.0, 0.07)),
                free(p!(0.0, 0.47), v!(0.0, -0.05), v!(0.0, 0.01)),
                mirrored(p!(0.0, 0.5), v!(0.0, 0.01))
            ].as_slice(), width, feathering, precision)
        }
    }
}