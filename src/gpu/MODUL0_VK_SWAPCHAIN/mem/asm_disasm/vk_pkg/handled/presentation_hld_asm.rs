//! vk_pkg **handled** — `PresentationDefaultRtCrg` assemble peels live on port;
//! **disassemble** + **recreate_extent** catalog (FIX-129 Handled rank).

use ash::khr::swapchain::Device as SwapchainDevice;
use ash::Device;

use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RenderPassTriangleRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::conv::port::{
    PresentationBfr, PresentationBfrAuto, PresentationTransportable,
};
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::handled::swapchain_hld_asm::SwapchainDefaultHandled;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::swapchain_prt::SwapchainPrt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::{
    PresentationDefaultRtCrg, SwapchainDefaultRtPkg,
};
use crate::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::presentation_destroy::destroy_presentation_gpu;
use crate::ModulResult;

/// Catalog — presentation product free + extent recreate (mesh/device stay app-owned).
pub trait PresentationDefaultHandled {
    /// Free FB / MSAA / depth / views / KHR (caller should idle device first).
    fn handled_disassemble(
        device_extrl: &Device,
        swapchain_loader_extrl: &SwapchainDevice,
        presentation: &mut PresentationDefaultRtCrg,
    );

    /// Destroy old present peels · new KHR at extent · re-import presentation lane in place.
    fn handled_recreate_extent(
        presentation: &mut PresentationDefaultRtCrg,
        boot: &SwapchainRtCrg,
        render_pass_triangle_rt_pkg: &RenderPassTriangleRtPkg,
        present_intent: SwapchainPrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
    ) -> ModulResult<()>;
}

impl PresentationDefaultHandled for PresentationDefaultRtCrg {
    fn handled_disassemble(
        device_extrl: &Device,
        swapchain_loader_extrl: &SwapchainDevice,
        presentation: &mut PresentationDefaultRtCrg,
    ) {
        destroy_presentation_gpu(device_extrl, swapchain_loader_extrl, presentation);
    }

    fn handled_recreate_extent(
        presentation: &mut PresentationDefaultRtCrg,
        boot: &SwapchainRtCrg,
        render_pass_triangle_rt_pkg: &RenderPassTriangleRtPkg,
        present_intent: SwapchainPrt,
        extent_width_stp: u32,
        extent_height_stp: u32,
    ) -> ModulResult<()> {
        let w = extent_width_stp.max(1);
        let h = extent_height_stp.max(1);
        let device = &boot.device_default_rt_pkg.device_extrl;
        let loader = &boot.swapchain_loader_default_rt_pkg.swapchain_loader_extrl;

        unsafe {
            let _ = device.device_wait_idle();
        }

        let sample_count = presentation.sample_count_default_rt_pkg.sample_count_op;
        let depth_format = presentation.depth_format_op;

        Self::handled_disassemble(device, loader, presentation);

        let new_khr = SwapchainDefaultRtPkg::handled_assemble(
            &boot.surface_default_rt_pkg,
            &boot.physical_device_default_rt_pkg,
            &boot.swapchain_loader_default_rt_pkg,
            present_intent,
            w,
            h,
        )?;

        let mut presentation_bfr = PresentationBfr::auto_assemble();
        PresentationBfr::import_for_asm6(
            &mut presentation_bfr,
            boot,
            render_pass_triangle_rt_pkg,
            new_khr,
            sample_count,
            depth_format,
        )?;

        *presentation = presentation_bfr
            .cargo_rt
            .take()
            .ok_or_else(|| "presentation: recreate cargo missing".to_string())?;
        Ok(())
    }
}
