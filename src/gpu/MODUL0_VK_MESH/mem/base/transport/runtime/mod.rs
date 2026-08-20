//! Port module `gpu/MODUL0_VK_MESH/mem/base/transport/runtime`.
//!
//! PTP import/export free functions (FIX-128/130).

pub mod line_gpu_default_rt_pkg;
/// Submodule `mesh_draw_default_rt_crg`.
/// Part of `gpu/MODUL0_VK_MESH/mem/base/transport/runtime` under the mem/conv/proc MCG canon.
pub mod mesh_draw_default_rt_crg;
/// Submodule `mesh_draw_default_rt_pkg`.
/// Part of `gpu/MODUL0_VK_MESH/mem/base/transport/runtime` under the mem/conv/proc MCG canon.
pub mod mesh_draw_default_rt_pkg;
/// Submodule `mesh_gpu_default_rt_pkg`.
///
/// Part of `gpu/MODUL0_VK_MESH/mem/base/transport/runtime` under the mem/conv/proc MCG canon.
pub mod mesh_gpu_default_rt_pkg;
/// Submodule `mesh_soa_rt_bfr`.
/// Part of `gpu/MODUL0_VK_MESH/mem/base/transport/runtime` under the mem/conv/proc MCG canon.
pub mod mesh_soa_rt_bfr;

// DomainMath peels on GPU bags (center · radius · push) live in proc.
// Empty RtPkg constructors live in mem/asm_disasm/vk_pkg/auto (FIX-136).
#[allow(unused_imports)]
use crate::gpu::MODUL0_VK_MESH::proc::processor::{
    line_gpu_rt_at_asm as _, mesh_gpu_rt_at_asm as _, mesh_push_rt_at_asm as _,
};
// Host SoA peels linked via CPU path when MeshSoaRtBfr is used.
#[allow(unused_imports)]
use crate::cpu::MODUL0_MESH::proc::processor::mesh_soa_at_asm as _;
