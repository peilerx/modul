//! Catalog — empty seed + slots · `MeshGpuBfrAuto`.

use crate::gpu::MODUL0_VK_MESH::mem::base::embedded::buffer::MeshGpuBfr;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshGpuDefaultRtPkg;
use crate::ModulResult;

/// `MeshGpuBfrAuto` — trait (mesh gpu bfr auto).
///
/// Auto-assemble catalog trait: pure construction without external peels (FIX-129 Auto rank).
/// Belongs to: mesh upload / solid draw MCG.
/// Module path context: `gpu/MODUL0_VK_MESH/mem/asm_disasm/vk_bfr/auto`.
pub trait MeshGpuBfrAuto: Sized {
    fn auto_assemble() -> Self;
    fn slot_ref<'a, T>(slot: &'a Option<T>, name: &'static str) -> ModulResult<&'a T> {
        slot.as_ref()
            .ok_or_else(|| format!("mesh_gpu_bfr: slot `{name}` empty"))
    }
    fn slot_take<T>(slot: &mut Option<T>, name: &'static str) -> ModulResult<T> {
        slot.take()
            .ok_or_else(|| format!("mesh_gpu_bfr: slot `{name}` empty (take)"))
    }
    fn mesh_gpu(&self) -> ModulResult<&MeshGpuDefaultRtPkg>;
}

impl MeshGpuBfrAuto for MeshGpuBfr {
    fn auto_assemble() -> Self {
        Self {
            mesh_gpu_default_rt_pkg: None,
        }
    }
    fn mesh_gpu(&self) -> ModulResult<&MeshGpuDefaultRtPkg> {
        Self::slot_ref(&self.mesh_gpu_default_rt_pkg, "mesh_gpu_default_rt_pkg")
    }
}
