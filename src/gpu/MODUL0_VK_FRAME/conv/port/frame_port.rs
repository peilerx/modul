//! Frame subject port — **factory-line order only** (swapchain calque).
//!
//! *Bfr · `embedded/buffer/` · slots · `vk_bfr/auto` · `import_for_asm3`.

use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_bfr::auto::frame_bfr_at_asm::FrameBfrAuto;
use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_crg::auto::frame_default_rt_crg_export::frame_export_asmed_render1;
use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_crg::handled::frame_default_rt_crg_hld_asm::FrameDefaultRtCrgHandled;
use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_pkg::auto::frame_fif_prt_at_asm::FrameFifDefaultStpAuto;
use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_pkg::auto::render_runtime_at_asm::RenderRuntimeDefaultAuto;
use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_pkg::handled::frame_sync_res_intsct_hld_asm::FrameSyncDefaultHandled;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::setup::frame_fif_default_stp_pkg::FrameFifDefaultStpPkg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::buffer::FrameBfr;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::prt::frame_fif_prt::FrameFifPrt;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameDefaultRtCrg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameRenderDefaultRtPkg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameSyncDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::DeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainCommandPoolDefaultRtPkg;
use crate::ModulResult;

/// Boot `import_for_asm3`: **3** assemblies · 2 atom + 1 cargo pack.
pub const IMPORT_FOR_ASM_FACTORY_LINE_N: u8 = 3;

/// `FrameTransportable` — trait (frame transportable).
///
/// Transportable surface: import/export peels for PTP slot-factory-line wiring.
/// Belongs to: frames-in-flight MCG.
/// Module path context: `gpu/MODUL0_VK_FRAME/conv/port`.
pub trait FrameTransportable {
    /// Frame factory-line · **3** = asm 1 sync · asm 2 render · asm 3 pack.
    /// Barter: device + command pool from swapchain boot cargo.
    fn import_for_asm3(
        bfr: &mut Self,
        frame_fif_prt: FrameFifPrt,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        swapchain_command_pool_default_rt_pkg: &SwapchainCommandPoolDefaultRtPkg,
    ) -> ModulResult<()>;

    /// Handled · FIF *Stp already on Bfr (`FrameBfrHandled`) · sync + render + pack.
    fn import_for_asm2_from_stp(
        bfr: &mut Self,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        swapchain_command_pool_default_rt_pkg: &SwapchainCommandPoolDefaultRtPkg,
    ) -> ModulResult<()>;

    fn export_asmed1(bfr: &Self) -> Option<&FrameDefaultRtCrg>;

    /// Export asmed render policy peel · **1** product.
    fn export_asmed_render1(bfr: &Self) -> Option<&FrameRenderDefaultRtPkg>;
}

impl FrameTransportable for FrameBfr {
    fn import_for_asm3(
        bfr: &mut Self,
        frame_fif_prt: FrameFifPrt,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        swapchain_command_pool_default_rt_pkg: &SwapchainCommandPoolDefaultRtPkg,
    ) -> ModulResult<()> {
        debug_assert_eq!(IMPORT_FOR_ASM_FACTORY_LINE_N, 3);

        // Intent → setup slot (Auto PortMatch · closed gestalt on FrameFifPrt)
        bfr.frame_fif_default_stp_pkg =
            Some(FrameFifDefaultStpPkg::auto_assemble(frame_fif_prt));

        Self::import_for_asm2_from_stp(
            bfr,
            device_default_rt_pkg,
            swapchain_command_pool_default_rt_pkg,
        )
    }

    fn import_for_asm2_from_stp(
        bfr: &mut Self,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        swapchain_command_pool_default_rt_pkg: &SwapchainCommandPoolDefaultRtPkg,
    ) -> ModulResult<()> {
        let _ = bfr.fif_stp()?;

        // asm 1 · atom · sync (Handled · frames_in_flight_stp)
        bfr.frame_sync_default_rt_pkg = Some(FrameSyncDefaultRtPkg::handled_assemble(
            &device_default_rt_pkg.device_extrl,
            swapchain_command_pool_default_rt_pkg.command_pool_extrl,
            bfr.fif_stp()?.frames_in_flight_stp,
        )?);

        // asm 2 · atom · render policy (Auto · fixed defaults)
        bfr.frame_render_default_rt_pkg = Some(FrameRenderDefaultRtPkg::auto_assemble());

        // pack cargo from bfr slots (Handled)
        let cargo_rt = FrameDefaultRtCrg::handled_assemble(bfr)?;
        bfr.cargo_rt = Some(cargo_rt);

        Ok(())
    }

    fn export_asmed1(bfr: &Self) -> Option<&FrameDefaultRtCrg> {
        bfr.cargo_rt.as_ref()
    }

    fn export_asmed_render1(bfr: &Self) -> Option<&FrameRenderDefaultRtPkg> {
        bfr.cargo_rt
            .as_ref()
            .map(frame_export_asmed_render1)
            .or(bfr.frame_render_default_rt_pkg.as_ref())
    }
}
