//! vk_pkg surface — one import: **vk::surface** MCU (FIX-120 · ¬ multi-gen soup).

use ash::khr::surface;
use ash::vk;

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::EntryDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::InstanceDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::boot_res_intsct_rt_pkgs::SurfaceDefaultRtPkg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::auto::surface_res_intsct_at_asm::{
    SurfaceKhrAuto, SurfaceLoaderAuto,
};
use crate::ModulResult;

/// Catalog — pack surface handle + loader.
pub trait SurfaceDefaultAuto {
    fn auto_assemble(
        entry_default_rt: &EntryDefaultRt,
        instance_default_rt: &InstanceDefaultRt,
        display_handle_extrl: raw_window_handle::RawDisplayHandle,
        window_handle_extrl: raw_window_handle::RawWindowHandle,
    ) -> ModulResult<SurfaceDefaultRtPkg>;
}

impl SurfaceDefaultAuto for SurfaceDefaultRtPkg {
    fn auto_assemble(
        entry_default_rt: &EntryDefaultRt,
        instance_default_rt: &InstanceDefaultRt,
        display_handle_extrl: raw_window_handle::RawDisplayHandle,
        window_handle_extrl: raw_window_handle::RawWindowHandle,
    ) -> ModulResult<SurfaceDefaultRtPkg> {
        let surface_extrl = vk::SurfaceKHR::auto_assemble(
            &entry_default_rt.entry_extrl,
            &instance_default_rt.instance_extrl,
            display_handle_extrl,
            window_handle_extrl,
        )?;
        let surface_loader_extrl = surface::Instance::auto_assemble(
            &entry_default_rt.entry_extrl,
            &instance_default_rt.instance_extrl,
        );
        Ok(Self {
            surface_extrl,
            surface_loader_extrl,
            desc: "vulkan_surface",
        })
    }
}
