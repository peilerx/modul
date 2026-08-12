//! Mesh GPU subject port · **import_for_asm1**.

use ash::vk;
use ash::{Device, Instance};

use crate::gpu::MODUL0_VK_MESH::mem::asm_disasm::vk_pkg::handled::mesh_gpu_hld_asm::MeshGpuDefaultHandled;
use crate::gpu::MODUL0_VK_MESH::mem::base::embedded::buffer::MeshGpuBfr;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::prt::mesh_draw_prt::MeshDrawPrt;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshGpuDefaultRtPkg;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_soa_rt_bfr::MeshSoaRtBfr;
use crate::ModulResult;

/// `IMPORT_FOR_ASM_FACTORY_LINE_N` — const (`IMPORT_FOR_ASM_FACTORY_LINE_N`).
/// Module path context: `gpu/MODUL0_VK_MESH/conv/port`.
pub const IMPORT_FOR_ASM_FACTORY_LINE_N: u8 = 1;

/// `MeshGpuTransportable` — trait (mesh gpu transportable).
///
/// Transportable surface: import/export peels for PTP slot-factory-line wiring.
/// Belongs to: mesh upload / solid draw MCG.
/// Module path context: `gpu/MODUL0_VK_MESH/conv/port`.
pub trait MeshGpuTransportable {
    fn import_for_asm1(
        bfr: &mut Self,
        mesh_draw_prt: MeshDrawPrt,
        device_extrl: &Device,
        instance_extrl: &Instance,
        physical_device_extrl: vk::PhysicalDevice,
        mesh_soa_rt_bfr: &MeshSoaRtBfr,
    ) -> ModulResult<()>;

    fn export_asmed1(bfr: &Self) -> Option<&MeshGpuDefaultRtPkg>;
}

impl MeshGpuTransportable for MeshGpuBfr {
    fn import_for_asm1(
        bfr: &mut Self,
        mesh_draw_prt: MeshDrawPrt,
        device_extrl: &Device,
        instance_extrl: &Instance,
        physical_device_extrl: vk::PhysicalDevice,
        mesh_soa_rt_bfr: &MeshSoaRtBfr,
    ) -> ModulResult<()> {
        debug_assert_eq!(IMPORT_FOR_ASM_FACTORY_LINE_N, 1);
        // asm 1/1 · mesh gpu product
        bfr.mesh_gpu_default_rt_pkg = Some(MeshGpuDefaultRtPkg::handled_assemble(
            device_extrl,
            instance_extrl,
            physical_device_extrl,
            mesh_soa_rt_bfr,
            mesh_draw_prt,
        )?);
        Ok(())
    }

    fn export_asmed1(bfr: &Self) -> Option<&MeshGpuDefaultRtPkg> {
        bfr.mesh_gpu_default_rt_pkg.as_ref()
    }
}
