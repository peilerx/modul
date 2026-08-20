//! Handled free / presentation recreate for `TandemBfr` (FIX-129).

use crate::gpu::MODUL0_VK_MESH::mem::asm_disasm::vk_pkg::handled::mesh_gpu_hld_asm::MeshGpuDefaultHandled;
use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::mesh_gpu_default_rt_pkg::MeshGpuDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::conv::port::{SwapchainBfr, SwapchainTransportable};
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::handled::presentation_hld_asm::PresentationDefaultHandled;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::PresentationDefaultRtCrg;
use crate::tandem::MODUL0_TANDEM::mem::tandem_bfr::TandemBfr;

/// Catalog — reverse product lifetime + present-lane recreate.
pub trait TandemDefaultHandled {
    fn handled_disassemble(bfr: &mut TandemBfr);

    fn handled_recreate_presentation_extent(
        bfr: &mut TandemBfr,
        width: u32,
        height: u32,
    ) -> crate::ModulResult<()>;
}

impl TandemDefaultHandled for TandemBfr {
    fn handled_disassemble(bfr: &mut TandemBfr) {
        let Some(swapchain_rt_crg) = SwapchainBfr::export_asmed1(&bfr.swapchain_bfr) else {
            return;
        };
        let device = &swapchain_rt_crg.device_default_rt_pkg.device_extrl;
        let loader = &swapchain_rt_crg
            .swapchain_loader_default_rt_pkg
            .swapchain_loader_extrl;
        unsafe {
            let _ = device.device_wait_idle();
        }
        crate::gpu::MODUL0_VK_DISPLAY::proc::display::soa_color_target::destroy_soa_color_target(
            device,
            &mut bfr.display_rt,
        );
        crate::gpu::MODUL0_VK_DISPLAY::proc::display::soa_color_target::destroy_soa_heat_image(
            device,
            &mut bfr.display_rt,
        );
        PresentationDefaultRtCrg::handled_disassemble(device, loader, &mut bfr.presentation_rt);
        MeshGpuDefaultRtPkg::handled_disassemble(device, &mut bfr.mesh_gpu_rt);
    }

    fn handled_recreate_presentation_extent(
        bfr: &mut TandemBfr,
        width: u32,
        height: u32,
    ) -> crate::ModulResult<()> {
        let swapchain_rt_crg = SwapchainBfr::export_asmed1(&bfr.swapchain_bfr)
            .ok_or_else(|| "tandem recreate: swapchain_rt_crg missing".to_string())?;

        let present = bfr.session_stp.present_prt_op;
        PresentationDefaultRtCrg::handled_recreate_extent(
            &mut bfr.presentation_rt,
            swapchain_rt_crg,
            &bfr.renderer_rt.render_pass_triangle_rt_pkg,
            present,
            width,
            height,
        )?;

        bfr.frame_rt.frame_sync_default_rt_pkg.current_frame_rt = 0;

        let device = &swapchain_rt_crg.device_default_rt_pkg.device_extrl;
        let inst = &swapchain_rt_crg.instance_default_rt.instance_extrl;
        let phys = swapchain_rt_crg
            .physical_device_default_rt_pkg
            .physical_device_extrl;
        let ext = bfr.presentation_rt.swapchain_default_rt_pkg.extent_rt;
        crate::gpu::MODUL0_VK_DISPLAY::proc::display::soa_color_target::update_soa_color_target(
            device,
            inst,
            phys,
            ext,
            &mut bfr.display_rt,
        )?;
        crate::gpu::MODUL0_VK_MESH::proc::processor::mesh_soa_bind::bind_soa_color_image(
            device,
            &bfr.renderer_rt,
            &bfr.display_rt,
        )?;
        Ok(())
    }
}

/// Free entry — Handled disassemble.
pub fn free_tandem(bfr: &mut TandemBfr) {
    TandemBfr::handled_disassemble(bfr);
}

/// Resize / OUT_OF_DATE — presentation-only recreate (uses `session_stp.present_prt_op`).
pub fn recreate_presentation_extent(
    bfr: &mut TandemBfr,
    width: u32,
    height: u32,
) -> crate::ModulResult<()> {
    TandemBfr::handled_recreate_presentation_extent(bfr, width, height)
}
