//! Host mesh runtime bags (structs only · M.Base ¬ impl).

pub mod mesh_soa_rt_bfr;
pub use mesh_soa_rt_bfr::MeshSoaRtBfr;

// Link DomainMath peels (empty · unit_cuboid · lattice) defined in proc.
#[allow(unused_imports)]
use crate::cpu::MODUL0_MESH::proc::processor::mesh_soa_at_asm as _;
