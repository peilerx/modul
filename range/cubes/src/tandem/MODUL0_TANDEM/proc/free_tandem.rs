//! `free_tandem` — reverse product lifetime.

use modul::gpu::MODUL0_VK_MESH::mem::asm_disasm::vk_pkg::auto::line_gpu_res_intsct_at_asm::LineGpuDefaultAuto;
use modul::gpu::MODUL0_VK_MESH::mem::asm_disasm::vk_pkg::handled::mesh_gpu_hld_asm::MeshGpuDefaultHandled;
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::line_gpu_default_rt_pkg::LineGpuDefaultRtPkg;
use modul::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshGpuDefaultRtPkg;
use modul::gpu::MODUL0_VK_SWAPCHAIN::conv::port::{SwapchainBfr, SwapchainTransportable};

use crate::tandem::MODUL0_TANDEM::mem::tandem_bfr::TandemBfr;

pub fn free_tandem(hub: &mut TandemBfr) {
    let boot = match SwapchainBfr::export_asmed1(&hub.swapchain_bfr) {
        Some(b) => b,
        None => return,
    };
    let dev = &boot.device_default_rt_pkg.device_extrl;
    unsafe {
        let _ = dev.device_wait_idle();
    }
    MeshGpuDefaultRtPkg::handled_disassemble(dev, &mut hub.mesh_gpu_rt);
    LineGpuDefaultRtPkg::auto_disassemble(dev, &mut hub.grid_line_rt);
}
