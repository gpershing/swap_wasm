use egui::{epaint::CubicBezierShape, Color32, Painter, Pos2, Shape, Stroke, Vec2};

pub enum ControlHandles {
    Mirrored(Vec2),
    Free(Vec2, Vec2)
}

pub struct ControlPoint {
    point: Pos2,
    handles: ControlHandles
}

impl ControlPoint {
    fn handle_in(&self) -> Vec2 {
        match self.handles {
            ControlHandles::Mirrored(h) => h,
            ControlHandles::Free(h, _) => h,
        }
    }

    fn handle_out(&self) -> Vec2 {
        match self.handles {
            ControlHandles::Mirrored(h) => Vec2 { x: -h.x, y: -h.y },
            ControlHandles::Free(_, h) => h,
        }
    }
}

const fn mirrored(point: Pos2, handle1: Vec2) -> ControlPoint {
    ControlPoint { point, handles: ControlHandles::Mirrored(handle1) }
}

const fn free(point: Pos2, handle1: Vec2, handle2: Vec2) -> ControlPoint {
    ControlPoint { point, handles: ControlHandles::Free(handle1, handle2) }
}

pub struct Segment<const N: usize> {
    points: [ControlPoint; N]
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

pub const SEGMENT_C0: Segment<5> = Segment {
    points: [
        mirrored(p!(0.5, 0.0), v!(0.01, 0.0)),
        free(p!(0.47, 0.0), v!(0.01, 0.0), v!(-0.05, 0.0)),
        free(p!(0.34, -0.05), v!(0.07, 0.0), v!(-0.12, 0.0)),
        free(p!(0.0, 0.14), v!(0.12, 0.0), v!(-0.077, 0.0)),
        mirrored(p!(-0.14, 0.0), v!(0.0, 0.077))
    ]
};

fn transform(p: Pos2, center: Pos2, cos: f32, sin: f32, scale: Pos2) -> Pos2 {
    Pos2 {
        x: ((p.x * scale.x) * cos + (p.y * scale.y) * sin) + center.x,
        y: ((p.x * scale.x) * -sin + (p.y * scale.y) * cos) + center.y
    }
}

fn transformv(v: Vec2, cos: f32, sin: f32, scale: Pos2) -> Vec2 {
    Vec2 {
        x: (v.x * scale.x) * cos + (v.y * scale.y) * sin,
        y: (v.x * scale.x) * -sin + (v.y * scale.y) * cos
    }
}

impl<const N: usize> Segment<N> {
    pub fn draw(&self, painter: &Painter, center: Pos2, rotation: f32, scale: Pos2) {
        let c = f32::cos(rotation);
        let s = f32::sin(rotation);
        for idx in 1..N {
            let from = transform(self.points[idx - 1].point, center, c, s, scale);
            let from_handle = from + transformv(self.points[idx - 1].handle_out(), c, s, scale);
            let to = transform(self.points[idx].point, center, c, s, scale);
            let to_handle = to + transformv(self.points[idx].handle_in(), c, s, scale);
            painter.add(
                Shape::CubicBezier(CubicBezierShape::from_points_stroke(
                    [
                        from, from_handle, to_handle, to
                    ],
                    false,
                    Color32::TRANSPARENT,
                    Stroke { width: 1.0, color: Color32::BLUE })));
        }
    }
}