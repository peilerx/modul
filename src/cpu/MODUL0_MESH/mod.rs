//! # `MODUL0_MESH` — host mesh MCG (CPU lane)
//!
//! | Layer | Content |
//! |-------|---------|
//! | `mem/base` | `MeshSoaRtBfr` struct only |
//! | `proc/processor` | DomainMath: cuboid · lattice · pack instances · steel/index bytes |
//!
//! GPU VBO upload: `gpu::MODUL0_VK_MESH`.

pub mod mem;
pub mod proc;
