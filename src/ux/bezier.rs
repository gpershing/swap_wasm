use egui::{Color32, Mesh, Pos2, Vec2};

#[derive(Debug, Clone, Copy)]
pub enum CubicBezierControl {
    Mirrored(Vec2),
    Free(Vec2, Vec2)
}

#[derive(Debug, Clone, Copy)]
pub struct CubicBezierPoint {
    pub point: Pos2,
    pub handles: CubicBezierControl
}

impl CubicBezierPoint {
    fn handle_in(&self) -> Vec2 {
        match self.handles {
            CubicBezierControl::Mirrored(h) => Vec2 { x: -h.x, y: -h.y },
            CubicBezierControl::Free(h, _) => h,
        }
    }

    fn handle_out(&self) -> Vec2 {
        match self.handles {
            CubicBezierControl::Mirrored(h) => h,
            CubicBezierControl::Free(_, h) => h,
        }
    }
}

pub fn compute(curve: [Pos2; 4], t: f32) -> Pos2 {
    let s = 1.0 - t;
    curve[0] * s * s * s 
    + curve[1].to_vec2() * s * s * t * 3.0
    + curve[2].to_vec2() * s * t * t * 3.0
    + curve[3].to_vec2() * t * t * t
}

pub fn generate_lut(curve: [Pos2; 4], steps: usize) -> Vec<Pos2> {
    let s_inv = 1.0 / (steps as f32);
    (0..=steps).map(|step| compute(curve, (step as f32) * s_inv)).collect()
}

pub fn get_length_from_lut(lut: &[Pos2]) -> f32 {
    lut.iter().skip(1).fold((0.0, lut[0]), |(acc, prev), next| {
        (acc + prev.distance(*next), *next)
    }).0
}

pub struct CubicBezierMeshVertex {
    position: Pos2,
    t: f32
}

pub struct CubicBezierMesh {
    vertices: Vec<CubicBezierMeshVertex>
}

// Assumes this layout:
// 0 4 8 
// 1 5 9
// 2 6 10
// 3 7 11
fn sewn_triangles_with_feathering(len: u32) -> impl Iterator<Item = [u32; 3]> {
    let column_count = (len - 4) / 4;
    (0..column_count).flat_map(|col_idx| {
        let base = col_idx * 4;
        [
            [base, base + 1, base + 4],
            [base + 1, base + 5, base + 4],
            [base + 1, base + 2, base + 5],
            [base + 2, base + 6, base + 5],
            [base + 2, base + 3, base + 6],
            [base + 3, base + 7, base + 6]
        ]
    })
}

impl CubicBezierMesh {
    pub fn new(curve: &[CubicBezierPoint], width: f32, feathering: f32, precision: f32) -> Self {
        const LUT_LEN: usize = 10;

        if curve.len() < 2 {
            panic!()
        }

        let mut beziers = Vec::new();
        for idx in 1..curve.len() {
            beziers.push([curve[idx - 1].point, curve[idx - 1].point + curve[idx - 1].handle_out(), curve[idx].point + curve[idx].handle_in(), curve[idx].point]);
        }
        let mut luts: Vec<_> = beziers.iter().map(|c| generate_lut(*c, LUT_LEN)).collect();
        let lengths: Vec<_> = luts.iter().map(|lut| get_length_from_lut(lut)).collect();
        let total_length: f32 = lengths.iter().sum();
        let total_length_inv = 1.0 / total_length;

        let half_width = width * 0.5;
        let feather_width = half_width + feathering * 0.5;

        let mut vertices = Vec::new();
        let previous_normal: Vec2 = (beziers[0][1] - beziers[0][0]).rot90().normalized();
        let mut previous_vertex: Pos2 = beziers[0][0];
        vertices.push(CubicBezierMeshVertex { position: previous_vertex + previous_normal * feather_width, t: 0.0 });
        vertices.push(CubicBezierMeshVertex { position: previous_vertex + previous_normal * half_width, t: 0.0 });
        vertices.push(CubicBezierMeshVertex { position: previous_vertex - previous_normal * half_width, t: 0.0 });
        vertices.push(CubicBezierMeshVertex { position: previous_vertex - previous_normal * feather_width, t: 0.0 });

        let mut at_t = 0.0;
        for (index, curve) in beziers.iter().enumerate() {
            let length = lengths[index];
            if length > precision * (LUT_LEN as f32) {
                luts[index] = generate_lut(*curve, (length / precision).ceil() as usize)
            }
            let lut = &luts[index];
            let lut_idx_to_t = length / (lut.len() as f32) * total_length_inv;
            for (lut_index, vertex) in lut.iter().enumerate() {
                let direction = if lut_index + 1 < lut.len() {
                    let next_vertex = lut[lut_index + 1];
                    (next_vertex - previous_vertex) * 0.5
                }
                else {
                    *vertex - curve[2]
                };
                let normal = direction.rot90().normalized();
                let t = at_t + (lut_index as f32) * lut_idx_to_t;

                vertices.push(CubicBezierMeshVertex { position: *vertex + normal * feather_width, t });
                vertices.push(CubicBezierMeshVertex { position: *vertex + normal * half_width, t });
                vertices.push(CubicBezierMeshVertex { position: *vertex - normal * half_width, t });
                vertices.push(CubicBezierMeshVertex { position: *vertex - normal * feather_width, t });

                previous_vertex = *vertex;
            }
            at_t += length * total_length_inv;
        }

        Self { vertices }
    }

    pub fn get_mesh(&self, transform: impl Fn(Pos2) -> Pos2, color_fn: impl Fn(f32) -> Color32) -> Mesh {
        let mut mesh = Mesh::default();
        mesh.reserve_vertices(self.vertices.len());
        mesh.reserve_triangles(self.vertices.len() - 2);
        for (index, vtx) in self.vertices.iter().enumerate() {
            if index % 4 == 0 || index % 4 == 3 {
                mesh.colored_vertex(transform(vtx.position), Color32::TRANSPARENT);
            }
            else {
                mesh.colored_vertex(transform(vtx.position), color_fn(vtx.t));
            }
        }
        for tri in sewn_triangles_with_feathering(self.vertices.len() as u32) {
            mesh.add_triangle(tri[0], tri[1], tri[2]);
        }
        mesh
    }
}