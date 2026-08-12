//! Presentation subject port — swapchain calque · **`import_for_asm6`**.

use ash::vk;

use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::runtime::render_res_intsct_rt_pkgs::RenderPassTriangleRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_bfr::auto::presentation_bfr_at_asm::PresentationBfrAuto;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_crg::handled::presentation_default_rt_crg_hld_asm::PresentationDefaultRtCrgHandled;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::auto::swapchain_image_views_at_asm::SwapchainImageViewsDefaultAuto;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::handled::depth_images_hld_asm::DepthImagesDefaultHandled;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::handled::framebuffer_hld_asm::FramebufferDefaultHandled;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::handled::msaa_color_hld_asm::MsaaColorDefaultHandled;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_pkg::handled::sample_count_hld_asm::SampleCountDefaultHandled;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::buffer::PresentationBfr;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::DepthImagesDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::FramebufferDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::MsaaColorDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::PresentationDefaultRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SampleCountDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::SwapchainImageViewsDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::present_res_intsct_stp_pkgs::PresentationDefaultStpPkg;
use crate::ModulResult;

/// Present lane factory-line · **6** = 5 atom + 1 pack.
pub const IMPORT_FOR_ASM_FACTORY_LINE_N: u8 = 6;

/// `PresentationTransportable` — trait (presentation transportable).
///
/// Transportable surface: import/export peels for PTP slot-factory-line wiring.
/// Belongs to: swapchain / device bootstrap MCG.
/// Module path context: `gpu/MODUL0_VK_SWAPCHAIN/conv/port`.
pub trait PresentationTransportable {
    /// Barter: boot cargo · pre-asmed KHR · render pass · present setup levers.
    fn import_for_asm6(
        bfr: &mut Self,
        swapchain_rt_crg: &SwapchainRtCrg,
        render_pass_triangle_rt_pkg: &RenderPassTriangleRtPkg,
        swapchain_default_rt_pkg: SwapchainDefaultRtPkg,
        sample_count_op: vk::SampleCountFlags,
        depth_format_op: vk::Format,
    ) -> ModulResult<()>;

    fn export_asmed1(bfr: &Self) -> Option<&PresentationDefaultRtCrg>;
}

impl PresentationTransportable for PresentationBfr {
    fn import_for_asm6(
        bfr: &mut Self,
        swapchain_rt_crg: &SwapchainRtCrg,
        render_pass_triangle_rt_pkg: &RenderPassTriangleRtPkg,
        swapchain_default_rt_pkg: SwapchainDefaultRtPkg,
        sample_count_op: vk::SampleCountFlags,
        depth_format_op: vk::Format,
    ) -> ModulResult<()> {
        debug_assert_eq!(IMPORT_FOR_ASM_FACTORY_LINE_N, 6);

        bfr.presentation_default_stp_pkg = Some(PresentationDefaultStpPkg {
            sample_count_op,
            depth_format_op,
            desc: "presentation_lane_stp",
        });
        bfr.swapchain_default_rt_pkg = Some(swapchain_default_rt_pkg);

        // asm 1/6 · image views
        bfr.swapchain_image_views_default_rt_pkg =
            Some(SwapchainImageViewsDefaultRtPkg::auto_assemble(
                &swapchain_rt_crg.device_default_rt_pkg,
                bfr.khr()?,
            )?);

        // asm 2/6 · sample count
        bfr.sample_count_default_rt_pkg = Some(SampleCountDefaultRtPkg::handled_assemble(
            bfr.stp()?.sample_count_op,
        ));

        // asm 3/6 · depth
        bfr.depth_images_default_rt_pkg = Some(DepthImagesDefaultRtPkg::handled_assemble(
            &swapchain_rt_crg.instance_default_rt,
            &swapchain_rt_crg.physical_device_default_rt_pkg,
            &swapchain_rt_crg.device_default_rt_pkg,
            bfr.khr()?,
            bfr.sample()?,
            bfr.stp()?.depth_format_op,
        )?);

        // asm 4/6 · msaa color
        bfr.msaa_color_default_rt_pkg = Some(MsaaColorDefaultRtPkg::handled_assemble(
            &swapchain_rt_crg.instance_default_rt,
            &swapchain_rt_crg.physical_device_default_rt_pkg,
            &swapchain_rt_crg.device_default_rt_pkg,
            bfr.khr()?,
            bfr.sample()?,
            bfr.stp()?.sample_count_op,
        )?);

        // asm 5/6 · framebuffer
        bfr.framebuffer_default_rt_pkg = Some(FramebufferDefaultRtPkg::handled_assemble(
            &swapchain_rt_crg.device_default_rt_pkg,
            bfr.khr()?,
            bfr.views()?,
            bfr.depth()?,
            bfr.msaa()?,
            render_pass_triangle_rt_pkg,
            bfr.stp()?.sample_count_op,
        )?);

        // asm 6/6 · pack
        let cargo_rt = PresentationDefaultRtCrg::handled_assemble(bfr)?;
        bfr.cargo_rt = Some(cargo_rt);
        Ok(())
    }

    fn export_asmed1(bfr: &Self) -> Option<&PresentationDefaultRtCrg> {
        bfr.cargo_rt.as_ref()
    }
}
