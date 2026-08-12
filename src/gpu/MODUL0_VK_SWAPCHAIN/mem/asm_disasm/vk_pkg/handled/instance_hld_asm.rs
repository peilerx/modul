use raw_window_handle::RawDisplayHandle;

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::EntryDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::InstanceDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::handled::instance_res_intsct_hld_asm::InstanceHandled;
use crate::ModulResult;

/// Catalog — pack `ash::Instance` into embedded bag (FIX-120).
/// `display_handle` selects WSI extensions (Wayland / X11 / …).
pub trait InstanceDefaultHandled {
    fn handled_assemble(
        entry_default_rt: &EntryDefaultRt,
        validation_layers_stp: bool,
        display_handle_extrl: RawDisplayHandle,
    ) -> ModulResult<InstanceDefaultRt>;
}

impl InstanceDefaultHandled for InstanceDefaultRt {
    fn handled_assemble(
        entry_default_rt: &EntryDefaultRt,
        validation_layers_stp: bool,
        display_handle_extrl: RawDisplayHandle,
    ) -> ModulResult<InstanceDefaultRt> {
        let instance_extrl = ash::Instance::handled_assemble(
            &entry_default_rt.entry_extrl,
            validation_layers_stp,
            display_handle_extrl,
        )?;
        Ok(Self {
            instance_extrl,
            desc: "vulkan_instance",
        })
    }
}
