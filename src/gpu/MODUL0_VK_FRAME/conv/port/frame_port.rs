//! Frame subject port — **factory-line order only** (swapchain calque).
//!
//! *Bfr · `embedded/buffer/` · slots · `vk_bfr/auto` · `import_for_asm3`.

use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_bfr::auto::frame_bfr_at_asm::FrameBfrAuto;
use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_crg::handled::frame_default_rt_crg_hld_asm::FrameDefaultRtCrgHandled;
use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_pkg::auto::render_runtime_at_asm::RenderRuntimeDefaultAuto;
use crate::gpu::MODUL0_VK_FRAME::mem::asm_disasm::vk_pkg::handled::frame_sync_res_intsct_hld_asm::FrameSyncDefaultHandled;
use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::buffer::FrameBfr;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::prt::frame_fif_prt::FrameFifPrt;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameDefaultRtCrg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameRenderDefaultRtPkg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameSyncDefaultRtPkg;
use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::setup::frame_fif_default_stp_pkg::FrameFifDefaultStpPkg;
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

        // Intent → setup slot (closed gestalt)
        bfr.frame_fif_default_stp_pkg = Some(match frame_fif_prt {
            FrameFifPrt::TripleBuffered => FrameFifDefaultStpPkg {
                frames_in_flight_stp: 3,
                fences_signaled_stp: true,
                primary_command_buffers_stp: true,
                desc: "frame_fif_triple_buffered",
            },
            FrameFifPrt::DoubleBuffered => FrameFifDefaultStpPkg {
                frames_in_flight_stp: 2,
                fences_signaled_stp: true,
                primary_command_buffers_stp: true,
                desc: "frame_fif_double_buffered",
            },
            FrameFifPrt::SingleBuffered => FrameFifDefaultStpPkg {
                frames_in_flight_stp: 1,
                fences_signaled_stp: true,
                primary_command_buffers_stp: true,
                desc: "frame_fif_single_buffered",
            },
        });

        // asm 1/3 · atom · sync
        bfr.frame_sync_default_rt_pkg = Some(FrameSyncDefaultRtPkg::handled_assemble(
            &device_default_rt_pkg.device_extrl,
            swapchain_command_pool_default_rt_pkg.command_pool_extrl,
            bfr.fif_stp()?.frames_in_flight_stp,
        )?);

        // asm 2/3 · atom · render policy
        bfr.frame_render_default_rt_pkg = Some(FrameRenderDefaultRtPkg::auto_assemble());

        // asm 3/3 · pack cargo from bfr slots
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
            .map(FrameDefaultRtCrg::export_asmed_render1)
            .or(bfr.frame_render_default_rt_pkg.as_ref())
    }
}
