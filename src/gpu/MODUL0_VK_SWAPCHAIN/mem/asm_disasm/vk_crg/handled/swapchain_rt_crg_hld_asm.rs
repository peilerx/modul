//! vk_crg **handled** — pack cargo from **bfr slots** (takes inside · ¬ port let theater).
//! ¬ create entry/device inside this method.

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk_bfr::auto::swapchain_bfr_at_asm::SwapchainBfrAuto;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::buffer::SwapchainBfr;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::swapchain_assembly_prt::SwapchainAssemblyPrt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SwapchainRtCrg;
use crate::ModulResult;

/// `SwapchainRtCrgHandled` — trait (swapchain rt crg handled).
/// Handled-assemble catalog trait: construction from imported knobs / peels (FIX-129 Handled rank).
/// Belongs to: swapchain / device bootstrap MCG.
/// Module path context: `gpu/MODUL0_VK_SWAPCHAIN/mem/asm_disasm/vk_crg/handled`.
pub trait SwapchainRtCrgHandled {
    /// Pack boot cargo from atom slots already filled on `bfr` (asm 1..7).
    fn handled_assemble(
        bfr: &mut SwapchainBfr,
        assembly_intent: SwapchainAssemblyPrt,
    ) -> ModulResult<SwapchainRtCrg>;
}

impl SwapchainRtCrgHandled for SwapchainRtCrg {
    fn handled_assemble(
        bfr: &mut SwapchainBfr,
        assembly_intent: SwapchainAssemblyPrt,
    ) -> ModulResult<SwapchainRtCrg> {
        let desc = match assembly_intent {
            SwapchainAssemblyPrt::GraphicsPresentValidation => {
                "swapchain_rt_crg_graphics_present_validation"
            }
            SwapchainAssemblyPrt::GraphicsPresentNoValidation => {
                "swapchain_rt_crg_graphics_present_no_validation"
            }
        };

        // surface stays on bfr (not taken) · atoms move into cargo
        let _surface = bfr.surface_window()?;

        Ok(SwapchainRtCrg {
            entry_default_rt: <SwapchainBfr as SwapchainBfrAuto>::slot_take(
                &mut bfr.entry_default_rt,
                "entry_default_rt",
            )?,
            instance_default_rt: <SwapchainBfr as SwapchainBfrAuto>::slot_take(
                &mut bfr.instance_default_rt,
                "instance_default_rt",
            )?,
            surface_default_rt_pkg: <SwapchainBfr as SwapchainBfrAuto>::slot_take(
                &mut bfr.surface_default_rt_pkg,
                "surface_default_rt_pkg",
            )?,
            physical_device_default_rt_pkg: <SwapchainBfr as SwapchainBfrAuto>::slot_take(
                &mut bfr.physical_device_default_rt_pkg,
                "physical_device_default_rt_pkg",
            )?,
            device_default_rt_pkg: <SwapchainBfr as SwapchainBfrAuto>::slot_take(
                &mut bfr.device_default_rt_pkg,
                "device_default_rt_pkg",
            )?,
            swapchain_command_pool_default_rt_pkg: <SwapchainBfr as SwapchainBfrAuto>::slot_take(
                &mut bfr.swapchain_command_pool_default_rt_pkg,
                "swapchain_command_pool_default_rt_pkg",
            )?,
            swapchain_loader_default_rt_pkg: <SwapchainBfr as SwapchainBfrAuto>::slot_take(
                &mut bfr.swapchain_loader_default_rt_pkg,
                "swapchain_loader_default_rt_pkg",
            )?,
            desc,
        })
    }
}
