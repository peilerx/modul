//! MeshPushRt DomainMath (orbit camera · look knobs) · P.Processor.

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshPushRt;

/// Byte size of the push-constant block (must match GLSL `cubes.*`).
pub const MESH_PUSH_RT_SIZE: u32 = core::mem::size_of::<MeshPushRt>() as u32;


impl MeshPushRt {
    /// Byte size of the push-constant block (must match GLSL `cubes.*` and pipeline layout).
    /// `identity_steel` — function (identity steel).
    /// Public API entry for this module.
    /// Belongs to: mesh upload / solid draw MCG.
    #[must_use]
    pub const fn identity_steel(base_rgb: [f32; 3]) -> Self {
        Self {
            mvp: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
            light_dir: [0.48, 0.72, 0.50, 1.0],
            base_color: [base_rgb[0], base_rgb[1], base_rgb[2], 0.14],
            cam_pos: [0.0, 0.0, 4.0, 1.08],
            look: [0.55, 1.2, 0.85, 0.0],
            look2: [0.5, 1.1, 1.55, 1.08],
            look3: [0.0, 0.0, 0.0, 2.0],
        }
    }

    /// Orbit camera around mesh AABB; perspective + Vulkan Y-flip.
    #[must_use]
    pub fn from_orbit(
        center: [f32; 3],
        radius: f32,
        yaw: f32,
        pitch: f32,
        aspect: f32,
        base_rgb: [f32; 3],
    ) -> Self {
        let r = radius.max(0.25) * 2.6;
        let cy = yaw.cos();
        let sy = yaw.sin();
        let cp = pitch.cos();
        let sp = pitch.sin();
        let eye = [
            (r * cp).mul_add(sy, center[0]),
            center[1] + r * sp,
            (r * cp).mul_add(cy, center[2]),
        ];
        let view = look_at_rh(eye, center, [0.0, 1.0, 0.0]);
        let proj = perspective_vk(45.0_f32.to_radians(), aspect.max(0.1), 0.05, r * 8.0);
        let mvp = mat4_mul(proj, view);
        Self {
            mvp,
            light_dir: [0.55, 0.82, 0.42, 1.2],
            base_color: [base_rgb[0], base_rgb[1], base_rgb[2], 0.14],
            cam_pos: [eye[0], eye[1], eye[2], 1.08],
            look: [0.55, 1.25, 0.9, 0.55],
            look2: [0.55, 1.1, 0.0, 1.12],
            look3: [0.0, 0.0, 0.0, 2.0],
        }
    }

    /// Overlay 3D View look knobs (keeps MVP / eye from orbit).
    pub const fn apply_view3d_look(
        &mut self,
        metal_f0: f32,
        roughness: f32,
        specular: f32,
        env_intensity: f32,
        key_intensity: f32,
        fill_intensity: f32,
        rim_intensity: f32,
        brush_amount: f32,
        film_amount: f32,
        exposure: f32,
        contrast: f32,
        cavity_i: f32,
        ridge_i: f32,
        shadow_i: f32,
        shadow_soft: f32,
    ) {
        self.light_dir[3] = key_intensity.clamp(0.0, 2.5);
        self.base_color[3] = roughness.clamp(0.02, 0.9);
        self.cam_pos[3] = exposure.clamp(0.5, 2.0);
        self.look = [
            metal_f0.clamp(0.05, 1.0),
            specular.clamp(0.0, 2.5),
            env_intensity.clamp(0.0, 2.5),
            fill_intensity.clamp(0.0, 2.5),
        ];
        self.look2 = [
            rim_intensity.clamp(0.0, 2.5),
            brush_amount.clamp(0.0, 2.5),
            film_amount.clamp(0.0, 2.5),
            contrast.clamp(0.5, 2.0),
        ];
        self.look3 = [
            cavity_i.clamp(0.0, 2.0),
            ridge_i.clamp(0.0, 2.0),
            shadow_i.clamp(0.0, 2.0),
            shadow_soft.clamp(0.2, 8.0),
        ];
    }
}

fn look_at_rh(eye: [f32; 3], center: [f32; 3], up: [f32; 3]) -> [f32; 16] {
    let f = normalize([
        center[0] - eye[0],
        center[1] - eye[1],
        center[2] - eye[2],
    ]);
    let s = normalize(cross(f, up));
    let u = cross(s, f);
    // Column-major view matrix.
    [
        s[0],
        u[0],
        -f[0],
        0.0,
        s[1],
        u[1],
        -f[1],
        0.0,
        s[2],
        u[2],
        -f[2],
        0.0,
        -dot(s, eye),
        -dot(u, eye),
        dot(f, eye),
        1.0,
    ]
}

fn perspective_vk(fovy: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
    let f = 1.0 / (fovy * 0.5).tan();
    let nf = 1.0 / (near - far);
    // Vulkan NDC Y is down — flip Y scale.
    [
        f / aspect,
        0.0,
        0.0,
        0.0,
        0.0,
        -f,
        0.0,
        0.0,
        0.0,
        0.0,
        far * nf,
        -1.0,
        0.0,
        0.0,
        far * near * nf,
        0.0,
    ]
}

fn mat4_mul(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut o = [0.0f32; 16];
    for col in 0..4 {
        for row in 0..4 {
            o[col * 4 + row] = a[12 + row].mul_add(b[col * 4 + 3], a[8 + row].mul_add(b[col * 4 + 2], a[4 + row].mul_add(b[col * 4 + 1], a[row] * b[col * 4])));
        }
    }
    o
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[2].mul_add(-b[1], a[1] * b[2]),
        a[0].mul_add(-b[2], a[2] * b[0]),
        a[1].mul_add(-b[0], a[0] * b[1]),
    ]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[2].mul_add(b[2], a[1].mul_add(b[1], a[0] * b[0]))
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let l = v[2].mul_add(v[2], v[1].mul_add(v[1], v[0] * v[0])).sqrt().max(1e-8);
    [v[0] / l, v[1] / l, v[2] / l]
}
