//! Port module `gpu/MODUL0_VK_MESH/mem/base/transport`.
//!
//! PTP import/export free functions (FIX-128/130).

pub mod prt;
/// Submodule `runtime`.
/// Part of `gpu/MODUL0_VK_MESH/mem/base/transport` under the mem/conv/proc MCG canon.
pub mod runtime;
/// Submodule `setup`.
/// Part of `gpu/MODUL0_VK_MESH/mem/base/transport` under the mem/conv/proc MCG canon.
pub mod setup;

pub use prt::MeshDrawPrt;
pub use runtime::mesh_draw_default_rt_pkg::MeshDrawDefaultRtPkg;
pub use runtime::mesh_draw_default_rt_crg::MeshDrawDefaultRtCrg;
pub use runtime::line_gpu_default_rt_pkg::{CadLinePushRt, LineGpuDefaultRtPkg};
pub use runtime::mesh_gpu_default_rt_pkg::{CadSteelPushRt, MeshGpuDefaultRtPkg};
pub use runtime::mesh_soa_rt_bfr::MeshSoaRtBfr;
pub use setup::mesh_draw_default_stp_pkg::MeshDrawDefaultStpPkg;
