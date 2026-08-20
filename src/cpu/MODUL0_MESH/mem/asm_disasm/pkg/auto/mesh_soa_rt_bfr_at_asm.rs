//! Auto seed · `MeshSoaRtBfr` empty host SoA bag.
//!
//! Constructor of the host bag lives here — not in `proc/` (FIX-136).

use crate::cpu::MODUL0_MESH::mem::base::transport::runtime::mesh_soa_rt_bfr::MeshSoaRtBfr;

/// Catalog — empty host mesh SoA bag.
pub trait MeshSoaRtAuto {
    fn auto_assemble() -> Self;
}

impl MeshSoaRtAuto for MeshSoaRtBfr {
    fn auto_assemble() -> Self {
        Self {
            pos_xs: Vec::new(),
            pos_ys: Vec::new(),
            pos_zs: Vec::new(),
            indices: Vec::new(),
            inst_xs: Vec::new(),
            inst_ys: Vec::new(),
            inst_zs: Vec::new(),
            logical_count: 0,
            lattice_pitch: 1.25,
            desc: "mesh_soa_empty",
        }
    }
}
