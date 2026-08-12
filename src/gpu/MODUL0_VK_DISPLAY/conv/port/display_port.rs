//! Display subject port · **`import_for_asm5`** (swapchain calque).

use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_bfr::auto::display_bfr_at_asm::DisplayBfrAuto;
use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_crg::handled::display_default_rt_crg_hld_asm::DisplayDefaultRtCrgHandled;
use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_pkg::auto::display_input_at_asm::DisplayInputDefaultAuto;
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
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::setup::display_present_default_stp_pkg::DisplayPresentDefaultStpPkg;
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

        bfr.display_present_default_stp_pkg = Some(match display_present_prt {
            DisplayPresentPrt::DefaultPresent => DisplayPresentDefaultStpPkg {
                frames_in_flight_stp,
                clear_only_stp: false,
                bind_geometry_stp: true,
                desc: "display_present_default",
            },
            DisplayPresentPrt::ClearColorOnly => DisplayPresentDefaultStpPkg {
                frames_in_flight_stp,
                clear_only_stp: true,
                bind_geometry_stp: false,
                desc: "display_present_clear_only",
            },
            DisplayPresentPrt::RecordTriangle => DisplayPresentDefaultStpPkg {
                frames_in_flight_stp,
                clear_only_stp: false,
                bind_geometry_stp: true,
                desc: "display_present_record_triangle",
            },
        });

        // asm 1/5 · input
        bfr.display_input_default_rt_pkg = Some(DisplayInputDefaultRtPkg::auto_assemble());

        // asm 2/5 · command
        bfr.command_rt = Some(DisplayCommandDefaultRt::handled_assemble(
            &device_default_rt_pkg.device_extrl,
            swapchain_command_pool_default_rt_pkg.command_pool_extrl,
            bfr.stp()?.frames_in_flight_stp,
        ));

        // asm 3/5 · render lane state
        bfr.display_render_default_rt = Some(DisplayRenderDefaultRt::auto_assemble());

        // asm 4/5 · vulkan display session
        bfr.vulkan_display_default_rt = Some(VulkanDisplayDefaultRt::auto_assemble(
            bfr.command()?,
            bfr.render()?,
        ));

        // asm 5/5 · pack
        let cargo_rt = DisplayDefaultRtCrg::handled_assemble(bfr)?;
        bfr.cargo_rt = Some(cargo_rt);
        Ok(())
    }

    fn export_asmed1(bfr: &Self) -> Option<&DisplayDefaultRtCrg> {
        bfr.cargo_rt.as_ref()
    }
}
