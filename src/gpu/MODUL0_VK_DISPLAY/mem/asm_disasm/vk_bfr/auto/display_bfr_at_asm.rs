//! Catalog — empty seed + slots · `DisplayBfrAuto`.

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::embedded::buffer::DisplayBfr;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayCommandDefaultRt;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayInputDefaultRtPkg;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayRenderDefaultRt;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::VulkanDisplayDefaultRt;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::setup::display_present_default_stp_pkg::DisplayPresentDefaultStpPkg;
use crate::ModulResult;

/// `DisplayBfrAuto` — trait (display bfr auto).
///
/// Auto-assemble catalog trait: pure construction without external peels (FIX-129 Auto rank).
/// Belongs to: command-buffer record / display MCG.
/// Module path context: `gpu/MODUL0_VK_DISPLAY/mem/asm_disasm/vk_bfr/auto`.
pub trait DisplayBfrAuto: Sized {
    fn auto_assemble() -> Self;
    fn slot_ref<'a, T>(slot: &'a Option<T>, name: &'static str) -> ModulResult<&'a T> {
        slot.as_ref()
            .ok_or_else(|| format!("display_bfr: slot `{name}` empty"))
    }
    fn slot_take<T>(slot: &mut Option<T>, name: &'static str) -> ModulResult<T> {
        slot.take()
            .ok_or_else(|| format!("display_bfr: slot `{name}` empty (take)"))
    }
    fn stp(&self) -> ModulResult<&DisplayPresentDefaultStpPkg>;
    fn input(&self) -> ModulResult<&DisplayInputDefaultRtPkg>;
    fn command(&self) -> ModulResult<&DisplayCommandDefaultRt>;
    fn render(&self) -> ModulResult<&DisplayRenderDefaultRt>;
    fn vulkan(&self) -> ModulResult<&VulkanDisplayDefaultRt>;
    fn cargo(&self) -> ModulResult<&DisplayDefaultRtCrg>;
}

impl DisplayBfrAuto for DisplayBfr {
    fn auto_assemble() -> Self {
        Self {
            display_present_default_stp_pkg: None,
            display_input_default_rt_pkg: None,
            command_rt: None,
            display_render_default_rt: None,
            vulkan_display_default_rt: None,
            cargo_rt: None,
        }
    }
    fn stp(&self) -> ModulResult<&DisplayPresentDefaultStpPkg> {
        Self::slot_ref(
            &self.display_present_default_stp_pkg,
            "display_present_default_stp_pkg",
        )
    }
    fn input(&self) -> ModulResult<&DisplayInputDefaultRtPkg> {
        Self::slot_ref(
            &self.display_input_default_rt_pkg,
            "display_input_default_rt_pkg",
        )
    }
    fn command(&self) -> ModulResult<&DisplayCommandDefaultRt> {
        Self::slot_ref(&self.command_rt, "command_rt")
    }
    fn render(&self) -> ModulResult<&DisplayRenderDefaultRt> {
        Self::slot_ref(
            &self.display_render_default_rt,
            "display_render_default_rt",
        )
    }
    fn vulkan(&self) -> ModulResult<&VulkanDisplayDefaultRt> {
        Self::slot_ref(
            &self.vulkan_display_default_rt,
            "vulkan_display_default_rt",
        )
    }
    fn cargo(&self) -> ModulResult<&DisplayDefaultRtCrg> {
        Self::slot_ref(&self.cargo_rt, "cargo_rt")
    }
}
