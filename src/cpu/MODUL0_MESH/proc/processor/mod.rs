//! Host mesh DomainMath (P).
//!
//! Lattice / cuboid / instance pack — not M.Assembler.

pub mod mesh_soa_at_asm;

pub use mesh_soa_at_asm::{
    instance_count, pack_instance_soa_bytes, pack_instance_xyzw_bytes,
    solid_unit_cells_exterior_mesh, unit_cuboid, unit_cuboid_instanced_lattice,
    unit_cuboid_instanced_lattice_ex, unit_cuboid_instanced_solid_shell, unit_cuboid_lattice_meta,
    world_bounds_from_local,
};
pub mod pack_index_bytes;
pub mod pack_line_lists;
pub mod pack_steel_interleaved;
