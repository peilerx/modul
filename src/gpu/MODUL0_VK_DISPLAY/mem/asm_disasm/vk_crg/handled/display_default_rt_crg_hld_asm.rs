//! vk_crg **handled** — pack display cargo from bfr slots.

use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_bfr::auto::display_bfr_at_asm::DisplayBfrAuto;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::embedded::buffer::DisplayBfr;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use crate::ModulResult;

/// `DisplayDefaultRtCrgHandled` — trait (display default rt crg handled).
/// Handled-assemble catalog trait: construction from imported knobs / peels (FIX-129 Handled rank).
/// Belongs to: command-buffer record / display MCG.
/// Module path context: `gpu/MODUL0_VK_DISPLAY/mem/asm_disasm/vk_crg/handled`.
pub trait DisplayDefaultRtCrgHandled {
    fn handled_assemble(bfr: &mut DisplayBfr) -> ModulResult<DisplayDefaultRtCrg>;
}

impl DisplayDefaultRtCrgHandled for DisplayDefaultRtCrg {
    fn handled_assemble(bfr: &mut DisplayBfr) -> ModulResult<DisplayDefaultRtCrg> {
        Ok(DisplayDefaultRtCrg {
            display_input_default_rt_pkg: <DisplayBfr as DisplayBfrAuto>::slot_take(
                &mut bfr.display_input_default_rt_pkg,
                "display_input_default_rt_pkg",
            )?,
            command_rt: <DisplayBfr as DisplayBfrAuto>::slot_take(
                &mut bfr.command_rt,
                "command_rt",
            )?,
            display_render_default_rt: <DisplayBfr as DisplayBfrAuto>::slot_take(
                &mut bfr.display_render_default_rt,
                "display_render_default_rt",
            )?,
            vulkan_display_default_rt: <DisplayBfr as DisplayBfrAuto>::slot_take(
                &mut bfr.vulkan_display_default_rt,
                "vulkan_display_default_rt",
            )?,
            desc: "display_rt",
        })
    }
}
