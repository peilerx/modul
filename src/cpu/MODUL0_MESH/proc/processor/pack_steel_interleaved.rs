//! Pack SoA mesh → interleaved pos+nrm (flat face normals).

use crate::cpu::MODUL0_MESH::mem::base::transport::runtime::mesh_soa_rt_bfr::MeshSoaRtBfr;

/// One vertex: pos.xyz + nrm.xyz (24 bytes).
pub const STEEL_VERT_STRIDE: usize = 24;

/// Pack mesh for steel VBO/IBO.
///
/// Unit cuboid (8 pos · 36 idx) → hard-edge 24 verts (index reuse per face) for
/// mass instancing. General meshes still expand to flat triangle lists.
#[must_use]
pub fn pack_steel_flat_from_mesh(
    mesh: &MeshSoaRtBfr,
) -> (Vec<u8>, Vec<u32>, [f32; 3], [f32; 3]) {
    if mesh.pos_xs.len() == 8 && mesh.indices.len() == 36 {
        return pack_unit_cuboid_hard_edge(mesh);
    }
    pack_steel_expand_flat(mesh)
}

/// 6 faces × 4 corners · flat nrm · 36 indices with per-face reuse.
/// Corner order is validated so geometric normal == outward (fixes 3-face cull).
fn pack_unit_cuboid_hard_edge(
    mesh: &MeshSoaRtBfr,
) -> (Vec<u8>, Vec<u32>, [f32; 3], [f32; 3]) {
    // face corner indices into the 8 unit-cuboid corners (must match unit_cuboid())
    let faces: [([u32; 4], [f32; 3]); 6] = [
        ([0, 1, 2, 3], [0.0, 0.0, -1.0]), // -Z
        ([4, 5, 6, 7], [0.0, 0.0, 1.0]),  // +Z
        ([0, 4, 5, 1], [0.0, -1.0, 0.0]), // -Y
        ([3, 2, 6, 7], [0.0, 1.0, 0.0]),  // +Y
        ([0, 3, 7, 4], [-1.0, 0.0, 0.0]), // -X
        ([1, 5, 6, 2], [1.0, 0.0, 0.0]),  // +X
    ];
    let mut verts: Vec<f32> = Vec::with_capacity(24 * 6);
    let mut indices: Vec<u32> = Vec::with_capacity(36);
    let mut bmin = [f32::MAX; 3];
    let mut bmax = [f32::MIN; 3];
    for (corners, nrm) in faces {
        let p = |ci: u32| -> [f32; 3] {
            let i = ci as usize;
            [mesh.pos_xs[i], mesh.pos_ys[i], mesh.pos_zs[i]]
        };
        let mut c = [p(corners[0]), p(corners[1]), p(corners[2]), p(corners[3])];
        let e1 = [c[1][0] - c[0][0], c[1][1] - c[0][1], c[1][2] - c[0][2]];
        let e2 = [c[2][0] - c[0][0], c[2][1] - c[0][1], c[2][2] - c[0][2]];
        let cxp = [
            e1[2].mul_add(-e2[1], e1[1] * e2[2]),
            e1[0].mul_add(-e2[2], e1[2] * e2[0]),
            e1[1].mul_add(-e2[0], e1[0] * e2[1]),
        ];
        let dot = cxp[2].mul_add(nrm[2], cxp[1].mul_add(nrm[1], cxp[0] * nrm[0]));
        if dot < 0.0 {
            c.swap(1, 3);
        }
        let base = (verts.len() / 6) as u32;
        for q in &c {
            verts.extend_from_slice(&[q[0], q[1], q[2], nrm[0], nrm[1], nrm[2]]);
            bmin[0] = bmin[0].min(q[0]);
            bmin[1] = bmin[1].min(q[1]);
            bmin[2] = bmin[2].min(q[2]);
            bmax[0] = bmax[0].max(q[0]);
            bmax[1] = bmax[1].max(q[1]);
            bmax[2] = bmax[2].max(q[2]);
        }
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    (f32_slice_to_bytes(&verts), indices, bmin, bmax)
}

/// Expand indexed mesh to non-indexed triangle list with flat face normals.
fn pack_steel_expand_flat(
    mesh: &MeshSoaRtBfr,
) -> (Vec<u8>, Vec<u32>, [f32; 3], [f32; 3]) {
    let npos = mesh.pos_xs.len();
    let mut verts: Vec<f32> = Vec::with_capacity(mesh.indices.len() * 6);
    let mut indices: Vec<u32> = Vec::with_capacity(mesh.indices.len());
    let mut bmin = [f32::MAX; 3];
    let mut bmax = [f32::MIN; 3];

    let mut i = 0;
    while i + 2 < mesh.indices.len() {
        let ia = mesh.indices[i] as usize;
        let ib = mesh.indices[i + 1] as usize;
        let ic = mesh.indices[i + 2] as usize;
        i += 3;
        if ia >= npos || ib >= npos || ic >= npos {
            continue;
        }
        let ax = mesh.pos_xs[ia];
        let ay = mesh.pos_ys[ia];
        let az = mesh.pos_zs[ia];
        let bx = mesh.pos_xs[ib];
        let by = mesh.pos_ys[ib];
        let bz = mesh.pos_zs[ib];
        let cx = mesh.pos_xs[ic];
        let cy = mesh.pos_ys[ic];
        let cz = mesh.pos_zs[ic];
        let e1x = bx - ax;
        let e1y = by - ay;
        let e1z = bz - az;
        let e2x = cx - ax;
        let e2y = cy - ay;
        let e2z = cz - az;
        let mut nx = e1z.mul_add(-e2y, e1y * e2z);
        let mut ny = e1x.mul_add(-e2z, e1z * e2x);
        let mut nz = e1y.mul_add(-e2x, e1x * e2y);
        let len = nz.mul_add(nz, ny.mul_add(ny, nx * nx)).sqrt();
        if len < 1e-12 {
            continue;
        }
        nx /= len;
        ny /= len;
        nz /= len;
        for (px, py, pz) in [(ax, ay, az), (bx, by, bz), (cx, cy, cz)] {
            let base = (verts.len() / 6) as u32;
            verts.extend_from_slice(&[px, py, pz, nx, ny, nz]);
            indices.push(base);
            bmin[0] = bmin[0].min(px);
            bmin[1] = bmin[1].min(py);
            bmin[2] = bmin[2].min(pz);
            bmax[0] = bmax[0].max(px);
            bmax[1] = bmax[1].max(py);
            bmax[2] = bmax[2].max(pz);
        }
    }
    if verts.is_empty() {
        bmin = [0.0; 3];
        bmax = [1.0; 3];
    }
    (f32_slice_to_bytes(&verts), indices, bmin, bmax)
}

fn f32_slice_to_bytes(verts: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(std::mem::size_of_val(verts));
    for f in verts {
        bytes.extend_from_slice(&f.to_ne_bytes());
    }
    bytes
}
