//! Re-export host SoA mesh from CPU lane (GPU MCG must not own host lattice).
pub use crate::cpu::MODUL0_MESH::mem::base::transport::runtime::mesh_soa_rt_bfr::MeshSoaRtBfr;
