//! `MeshGpuBfr` — mesh GPU product slot (type only).

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshGpuDefaultRtPkg;

/// `MeshGpuBfr` — buffer / warehouse bag (mesh gpu bfr).
/// Memory-layer bag: owned fields, no product control flow.
/// Belongs to: mesh upload / solid draw MCG.
/// Module path context: `gpu/MODUL0_VK_MESH/mem/base/embedded/buffer`.
pub struct MeshGpuBfr {
    /// Nested package bag field `mesh_gpu_default_rt_pkg`.
    pub mesh_gpu_default_rt_pkg: Option<MeshGpuDefaultRtPkg>,
}
