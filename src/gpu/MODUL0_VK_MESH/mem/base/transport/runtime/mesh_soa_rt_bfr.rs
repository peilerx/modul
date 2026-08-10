//! Minimal SoA mesh bag for GPU upload (no CAD MCG dependency).

/// Positions + triangle indices (host) · optional per-instance world offsets.
pub struct MeshSoaRtBfr {
    /// Public field `pos_xs`.
    pub pos_xs: Vec<f32>,
    /// Public field `pos_ys`.
    pub pos_ys: Vec<f32>,
    /// Public field `pos_zs`.
    pub pos_zs: Vec<f32>,
    /// Public field `indices`.
    pub indices: Vec<u32>,
    /// World translation per instance (xyz). Empty → one instance at origin.
    pub inst_xs: Vec<f32>,
    /// Public field `inst_ys`.
    pub inst_ys: Vec<f32>,
    /// Public field `inst_zs`.
    pub inst_zs: Vec<f32>,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

impl MeshSoaRtBfr {
    /// `empty` — function (empty).
    /// Public API entry for this module.
    /// Belongs to: mesh upload / solid draw MCG.
    pub fn empty() -> Self {
        Self {
            pos_xs: Vec::new(),
            pos_ys: Vec::new(),
            pos_zs: Vec::new(),
            indices: Vec::new(),
            inst_xs: Vec::new(),
            inst_ys: Vec::new(),
            inst_zs: Vec::new(),
            desc: "mesh_soa_empty",
        }
    }

    /// Unit cuboid (axis-aligned 1×1×1 centered at origin) for VK study.
    pub fn unit_cuboid() -> Self {
        let p = [
            [-0.5f32, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, 0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [-0.5, 0.5, 0.5],
        ];
        let mut pos_xs = Vec::with_capacity(8);
        let mut pos_ys = Vec::with_capacity(8);
        let mut pos_zs = Vec::with_capacity(8);
        for q in p {
            pos_xs.push(q[0]);
            pos_ys.push(q[1]);
            pos_zs.push(q[2]);
        }
        // 12 tris, outward CCW when viewed from outside
        let indices = vec![
            0, 1, 2, 0, 2, 3, // -Z
            4, 6, 5, 4, 7, 6, // +Z
            0, 4, 5, 0, 5, 1, // -Y
            2, 6, 7, 2, 7, 3, // +Y
            0, 3, 7, 0, 7, 4, // -X
            1, 5, 6, 1, 6, 2, // +X
        ];
        Self {
            pos_xs,
            pos_ys,
            pos_zs,
            indices,
            inst_xs: Vec::new(),
            inst_ys: Vec::new(),
            inst_zs: Vec::new(),
            desc: "mesh_soa_unit_cuboid",
        }
    }

    /// One unit cuboid mesh + `count` instance translations on a compact 3D lattice.
    ///
    /// GPU path: `cmd_draw_indexed(36, instances, …)` — not expanded host geometry.
    ///
    /// `shell_only`: for solid packs (`pitch ≈ 1`) interior cubes are never visible —
    /// emit only the outer shell so raster cost is O(n²) while the volume still
    /// represents `count` logical cells (n³).
    pub fn unit_cuboid_instanced_lattice(count: usize, pitch: f32) -> Self {
        Self::unit_cuboid_instanced_lattice_ex(count, pitch, false)
    }

    /// Solid 100k-class block: tight pitch + shell instances (interior culled on host).
    pub fn unit_cuboid_instanced_solid_shell(logical_count: usize) -> Self {
        Self::unit_cuboid_instanced_lattice_ex(logical_count, 1.0, true)
    }

    /// Solid volume of `count` unit cells (pitch 1) as **one mesh of exterior faces only**.
    ///
    /// No interior geometry · no hollow shell cubes — only the 6 macro-sides tiled by
    /// unit-cell quads (looks like one big cube built from little planes).
    pub fn solid_unit_cells_exterior_mesh(count: usize) -> Self {
        let count = count.max(1);
        let nx = (count as f32).cbrt().ceil() as usize;
        let ny = nx.max(1);
        let nz = ((count + nx * ny - 1) / (nx * ny)).max(1);
        let plane = nx * ny;
        let occupied = |ix: isize, iy: isize, iz: isize| -> bool {
            if ix < 0 || iy < 0 || iz < 0 {
                return false;
            }
            let (ix, iy, iz) = (ix as usize, iy as usize, iz as usize);
            if ix >= nx || iy >= ny || iz >= nz {
                return false;
            }
            iz * plane + iy * nx + ix < count
        };

        let cx0 = (nx.saturating_sub(1) as f32) * 0.5;
        let cy0 = (ny.saturating_sub(1) as f32) * 0.5;
        let cz0 = (nz.saturating_sub(1) as f32) * 0.5;

        // (neighbor offset, outward normal, 4 corners local to cell center)
        // Corner order: CCW when viewed from outside along outward normal.
        let faces: [(isize, isize, isize, [f32; 3], [[f32; 3]; 4]); 6] = [
            (
                -1,
                0,
                0,
                [-1.0, 0.0, 0.0],
                [
                    [-0.5, -0.5, -0.5],
                    [-0.5, 0.5, -0.5],
                    [-0.5, 0.5, 0.5],
                    [-0.5, -0.5, 0.5],
                ],
            ),
            (
                1,
                0,
                0,
                [1.0, 0.0, 0.0],
                [
                    [0.5, -0.5, -0.5],
                    [0.5, -0.5, 0.5],
                    [0.5, 0.5, 0.5],
                    [0.5, 0.5, -0.5],
                ],
            ),
            (
                0,
                -1,
                0,
                [0.0, -1.0, 0.0],
                [
                    [-0.5, -0.5, -0.5],
                    [-0.5, -0.5, 0.5],
                    [0.5, -0.5, 0.5],
                    [0.5, -0.5, -0.5],
                ],
            ),
            (
                0,
                1,
                0,
                [0.0, 1.0, 0.0],
                [
                    [-0.5, 0.5, -0.5],
                    [0.5, 0.5, -0.5],
                    [0.5, 0.5, 0.5],
                    [-0.5, 0.5, 0.5],
                ],
            ),
            (
                0,
                0,
                -1,
                [0.0, 0.0, -1.0],
                [
                    [-0.5, -0.5, -0.5],
                    [0.5, -0.5, -0.5],
                    [0.5, 0.5, -0.5],
                    [-0.5, 0.5, -0.5],
                ],
            ),
            (
                0,
                0,
                1,
                [0.0, 0.0, 1.0],
                [
                    [-0.5, -0.5, 0.5],
                    [-0.5, 0.5, 0.5],
                    [0.5, 0.5, 0.5],
                    [0.5, -0.5, 0.5],
                ],
            ),
        ];

        let mut pos_xs = Vec::new();
        let mut pos_ys = Vec::new();
        let mut pos_zs = Vec::new();
        let mut indices = Vec::new();

        let mut linear = 0usize;
        'cells: for iz in 0..nz {
            for iy in 0..ny {
                for ix in 0..nx {
                    if linear >= count {
                        break 'cells;
                    }
                    linear += 1;
                    let ox = ix as f32 - cx0;
                    let oy = iy as f32 - cy0;
                    let oz = iz as f32 - cz0;
                    let iix = ix as isize;
                    let iiy = iy as isize;
                    let iiz = iz as isize;
                    for (dx, dy, dz, outward, corners) in faces {
                        if occupied(iix + dx, iiy + dy, iiz + dz) {
                            continue;
                        }
                        // 4 world corners
                        let mut w = [[0.0f32; 3]; 4];
                        for k in 0..4 {
                            w[k] = [
                                corners[k][0] + ox,
                                corners[k][1] + oy,
                                corners[k][2] + oz,
                            ];
                        }
                        // Force winding so geometric normal matches outward
                        // (Vulkan proj flips Y — still author in world CCW = outward).
                        let e1 = [w[1][0] - w[0][0], w[1][1] - w[0][1], w[1][2] - w[0][2]];
                        let e2 = [w[2][0] - w[0][0], w[2][1] - w[0][1], w[2][2] - w[0][2]];
                        let cxp = [
                            e1[1] * e2[2] - e1[2] * e2[1],
                            e1[2] * e2[0] - e1[0] * e2[2],
                            e1[0] * e2[1] - e1[1] * e2[0],
                        ];
                        let dot = cxp[0] * outward[0] + cxp[1] * outward[1] + cxp[2] * outward[2];
                        let order: [usize; 4] = if dot >= 0.0 {
                            [0, 1, 2, 3]
                        } else {
                            [0, 3, 2, 1]
                        };
                        let base = pos_xs.len() as u32;
                        for &oi in &order {
                            pos_xs.push(w[oi][0]);
                            pos_ys.push(w[oi][1]);
                            pos_zs.push(w[oi][2]);
                        }
                        indices.extend_from_slice(&[
                            base,
                            base + 1,
                            base + 2,
                            base,
                            base + 2,
                            base + 3,
                        ]);
                    }
                }
            }
        }

        Self {
            pos_xs,
            pos_ys,
            pos_zs,
            indices,
            inst_xs: Vec::new(),
            inst_ys: Vec::new(),
            inst_zs: Vec::new(),
            desc: "mesh_soa_solid_unit_cells_exterior",
        }
    }

    /// `unit_cuboid_instanced_lattice_ex` — function (unit cuboid instanced lattice ex).
    /// Public API entry for this module.
    /// Belongs to: mesh upload / solid draw MCG.
    pub fn unit_cuboid_instanced_lattice_ex(
        count: usize,
        pitch: f32,
        shell_only: bool,
    ) -> Self {
        let mut mesh = Self::unit_cuboid();
        let count = count.max(1);
        let pitch = if pitch.is_finite() && pitch > 0.0 {
            pitch
        } else {
            1.15
        };
        let nx = (count as f32).cbrt().ceil() as usize;
        let ny = nx.max(1);
        let nz = ((count + nx * ny - 1) / (nx * ny)).max(1);
        let cx0 = (nx.saturating_sub(1) as f32) * 0.5;
        let cy0 = (ny.saturating_sub(1) as f32) * 0.5;
        let cz0 = (nz.saturating_sub(1) as f32) * 0.5;

        let cap = if shell_only {
            // upper bound on shell cells
            let a = nx * ny;
            let b = nx * nz;
            let c = ny * nz;
            (2 * (a + b + c)).min(count)
        } else {
            count
        };
        mesh.inst_xs = Vec::with_capacity(cap);
        mesh.inst_ys = Vec::with_capacity(cap);
        mesh.inst_zs = Vec::with_capacity(cap);

        let mut placed = 0usize;
        'outer: for iz in 0..nz {
            for iy in 0..ny {
                for ix in 0..nx {
                    if placed >= count {
                        break 'outer;
                    }
                    let on_shell = ix == 0
                        || iy == 0
                        || iz == 0
                        || ix + 1 == nx
                        || iy + 1 == ny
                        || iz + 1 == nz;
                    if shell_only && !on_shell {
                        placed += 1;
                        continue;
                    }
                    mesh.inst_xs.push((ix as f32 - cx0) * pitch);
                    mesh.inst_ys.push((iy as f32 - cy0) * pitch);
                    mesh.inst_zs.push((iz as f32 - cz0) * pitch);
                    placed += 1;
                }
            }
        }
        // shell path skips interior without pushing — recount logical placed for fill
        if shell_only {
            // re-walk only shell (cleaner): rebuild if empty edge case
            if mesh.inst_xs.is_empty() && count > 0 {
                mesh.inst_xs.push(0.0);
                mesh.inst_ys.push(0.0);
                mesh.inst_zs.push(0.0);
            }
        }
        mesh.desc = if count == 1 {
            "mesh_soa_unit_cuboid"
        } else if shell_only {
            "mesh_soa_unit_cuboid_instanced_solid_shell"
        } else {
            "mesh_soa_unit_cuboid_instanced_lattice"
        };
        mesh
    }

    /// `instance_count` — function (instance count).
    /// Public API entry for this module.
    /// Belongs to: mesh upload / solid draw MCG.
    #[inline]
    pub fn instance_count(&self) -> usize {
        let n = self.inst_xs.len().min(self.inst_ys.len()).min(self.inst_zs.len());
        n.max(1)
    }

    /// Host-visible instance buffer bytes: xyzw stride 16 (w=0).
    pub fn pack_instance_xyzw_bytes(&self) -> Vec<u8> {
        let n = self.inst_xs.len().min(self.inst_ys.len()).min(self.inst_zs.len());
        if n == 0 {
            let mut out = Vec::with_capacity(16);
            for f in [0.0f32, 0.0, 0.0, 0.0] {
                out.extend_from_slice(&f.to_ne_bytes());
            }
            return out;
        }
        let mut out = Vec::with_capacity(n * 16);
        for i in 0..n {
            for f in [self.inst_xs[i], self.inst_ys[i], self.inst_zs[i], 0.0f32] {
                out.extend_from_slice(&f.to_ne_bytes());
            }
        }
        out
    }

    /// World AABB of mesh local AABB translated by every instance (orbit fit).
    pub fn world_bounds_from_local(&self, local_min: [f32; 3], local_max: [f32; 3]) -> ([f32; 3], [f32; 3]) {
        let n = self.inst_xs.len().min(self.inst_ys.len()).min(self.inst_zs.len());
        if n == 0 {
            return (local_min, local_max);
        }
        let mut bmin = [f32::MAX; 3];
        let mut bmax = [f32::MIN; 3];
        for i in 0..n {
            let ox = self.inst_xs[i];
            let oy = self.inst_ys[i];
            let oz = self.inst_zs[i];
            bmin[0] = bmin[0].min(local_min[0] + ox);
            bmin[1] = bmin[1].min(local_min[1] + oy);
            bmin[2] = bmin[2].min(local_min[2] + oz);
            bmax[0] = bmax[0].max(local_max[0] + ox);
            bmax[1] = bmax[1].max(local_max[1] + oy);
            bmax[2] = bmax[2].max(local_max[2] + oz);
        }
        (bmin, bmax)
    }
}
