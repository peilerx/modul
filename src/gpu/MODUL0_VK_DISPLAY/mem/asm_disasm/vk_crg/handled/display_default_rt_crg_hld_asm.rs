//! `vk_crg` **handled** — pack display cargo from bfr slots.

use ash::vk;

use crate::gpu::MODUL0_VK_DISPLAY::mem::asm_disasm::vk_bfr::auto::display_bfr_at_asm::DisplayBfrAuto;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::embedded::buffer::DisplayBfr;
use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use crate::ModulResult;

/// `DisplayDefaultRtCrgHandled` — trait (display default rt crg handled).
///
/// Handled-assemble catalog trait: construction from imported knobs / peels (FIX-129 Handled rank).
/// Belongs to: command-buffer record / display MCG.
/// Module path context: `gpu/MODUL0_VK_DISPLAY/mem/asm_disasm/vk_crg/handled`.
pub trait DisplayDefaultRtCrgHandled {
    fn handled_assemble(bfr: &mut DisplayBfr) -> ModulResult<DisplayDefaultRtCrg>;
}

impl DisplayDefaultRtCrgHandled for DisplayDefaultRtCrg {
    fn handled_assemble(bfr: &mut DisplayBfr) -> ModulResult<DisplayDefaultRtCrg> {
        Ok(Self {
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
            soa_color_image_extrl: vk::Image::null(),
            soa_color_memory_extrl: vk::DeviceMemory::null(),
            soa_color_view_extrl: vk::ImageView::null(),
            soa_color_extent_rt: vk::Extent2D::default(),
            soa_heat_image_extrl: vk::Image::null(),
            soa_heat_view_extrl: vk::ImageView::null(),
            soa_heat_memory_extrl: vk::DeviceMemory::null(),
            soa_heat_extent_rt: vk::Extent3D::default(),
            soa_heat_bytes_rt: 0,
            soa_heat_cleared_rt: false,
            heat_mouse_x_rt: 0.0,
            heat_mouse_y_rt: 0.0,
            heat_dt_rt: 1.0 / 60.0,
            heat_hold_rt: 0.0,
            heat_paint_rt: 0,
            heat_run_rt: 0,
            desc: "display_rt",
        })
    }
}
