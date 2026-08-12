//! Display subject port · **`import_for_asm5`** (swapchain calque).

use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_bfr::auto::display_bfr_at_asm::DisplayBfrAuto;
use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_crg::handled::display_default_rt_crg_hld_asm::DisplayDefaultRtCrgHandled;
use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_pkg::auto::display_input_at_asm::DisplayInputDefaultAuto;
use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_pkg::auto::display_present_prt_at_asm::DisplayPresentDefaultStpAuto;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::setup::display_present_default_stp_pkg::DisplayPresentDefaultStpPkg;
use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_pkg::handled::display_res_intsct_hld_asm::CommandDefaultHandled;
use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_pkg::handled::display_res_intsct_hld_asm::DisplayRenderDefaultAuto;
use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_pkg::handled::display_res_intsct_hld_asm::VulkanDisplayRuntimeDefaultAuto;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::embedded::buffer::DisplayBfr;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::prt::display_present_prt::DisplayPresentPrt;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayCommandDefaultRt;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayInputDefaultRtPkg;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayRenderDefaultRt;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::VulkanDisplayDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::DeviceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainCommandPoolDefaultRtPkg;
use crate::ModulResult;

/// `IMPORT_FOR_ASM_FACTORY_LINE_N` — const (`IMPORT_FOR_ASM_FACTORY_LINE_N`).
/// Module path context: `gpu/MODUL0_VK_DISPLAY/conv/port`.
pub const IMPORT_FOR_ASM_FACTORY_LINE_N: u8 = 5;

/// `DisplayTransportable` — trait (display transportable).
///
/// Transportable surface: import/export peels for PTP slot-factory-line wiring.
/// Belongs to: command-buffer record / display MCG.
/// Module path context: `gpu/MODUL0_VK_DISPLAY/conv/port`.
pub trait DisplayTransportable {
    fn import_for_asm5(
        bfr: &mut Self,
        display_present_prt: DisplayPresentPrt,
        frames_in_flight_stp: u32,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        swapchain_command_pool_default_rt_pkg: &SwapchainCommandPoolDefaultRtPkg,
    ) -> ModulResult<()>;

    /// Handled · present *Stp already on Bfr (`DisplayBfrHandled`).
    fn import_for_asm4_from_stp(
        bfr: &mut Self,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        swapchain_command_pool_default_rt_pkg: &SwapchainCommandPoolDefaultRtPkg,
    ) -> ModulResult<()>;

    fn export_asmed1(bfr: &Self) -> Option<&DisplayDefaultRtCrg>;
}

impl DisplayTransportable for DisplayBfr {
    fn import_for_asm5(
        bfr: &mut Self,
        display_present_prt: DisplayPresentPrt,
        frames_in_flight_stp: u32,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        swapchain_command_pool_default_rt_pkg: &SwapchainCommandPoolDefaultRtPkg,
    ) -> ModulResult<()> {
        debug_assert_eq!(IMPORT_FOR_ASM_FACTORY_LINE_N, 5);

        bfr.display_present_default_stp_pkg = Some(DisplayPresentDefaultStpPkg::auto_assemble(
            display_present_prt,
            frames_in_flight_stp,
        ));

        Self::import_for_asm4_from_stp(
            bfr,
            device_default_rt_pkg,
            swapchain_command_pool_default_rt_pkg,
        )
    }

    fn import_for_asm4_from_stp(
        bfr: &mut Self,
        device_default_rt_pkg: &DeviceDefaultRtPkg,
        swapchain_command_pool_default_rt_pkg: &SwapchainCommandPoolDefaultRtPkg,
    ) -> ModulResult<()> {
        let _ = bfr.stp()?;

        // asm 1 · input (Auto · fixed defaults)
        bfr.display_input_default_rt_pkg = Some(DisplayInputDefaultRtPkg::auto_assemble());

        // asm 2 · command (Handled · frames_in_flight_stp)
        bfr.command_rt = Some(DisplayCommandDefaultRt::handled_assemble(
            &device_default_rt_pkg.device_extrl,
            swapchain_command_pool_default_rt_pkg.command_pool_extrl,
            bfr.stp()?.frames_in_flight_stp,
        ));

        // asm 3 · render lane state (Auto)
        bfr.display_render_default_rt = Some(DisplayRenderDefaultRt::auto_assemble());

        // asm 4 · vulkan display session (Auto · peels only)
        bfr.vulkan_display_default_rt = Some(VulkanDisplayDefaultRt::auto_assemble(
            bfr.command()?,
            bfr.render()?,
        ));

        // pack (Handled)
        let cargo_rt = DisplayDefaultRtCrg::handled_assemble(bfr)?;
        bfr.cargo_rt = Some(cargo_rt);
        Ok(())
    }

    fn export_asmed1(bfr: &Self) -> Option<&DisplayDefaultRtCrg> {
        bfr.cargo_rt.as_ref()
    }
}
