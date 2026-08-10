//! vk_pkg debug messenger — one import: **vk::debug_utils** MCU.

use ash::ext::debug_utils;
use ash::vk;

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::DebugMessengerDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::EntryDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::embedded::runtime::boot_res_intsct_rt::InstanceDefaultRt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::auto::debug_msg_res_intsct_at_asm::{
    DebugMsgLoaderAuto, DebugMsgMessengerAuto,
};
use crate::ModulResult;

/// `DebugMessengerDefaultAuto` — trait (debug messenger default auto).
/// Auto-assemble catalog trait: pure construction without external peels (FIX-129 Auto rank).
/// Belongs to: swapchain / device bootstrap MCG.
/// Module path context: `gpu/MODUL0_VK_SWAPCHAIN/mem/asm_disasm/vk_pkg/auto`.
pub trait DebugMessengerDefaultAuto {
    fn auto_assemble(
        entry_default_rt: &EntryDefaultRt,
        instance_default_rt: &InstanceDefaultRt,
    ) -> ModulResult<DebugMessengerDefaultRt>;
}

impl DebugMessengerDefaultAuto for DebugMessengerDefaultRt {
    fn auto_assemble(
        entry_default_rt: &EntryDefaultRt,
        instance_default_rt: &InstanceDefaultRt,
    ) -> ModulResult<DebugMessengerDefaultRt> {
        let debug_utils_extrl = debug_utils::Instance::auto_assemble(
            &entry_default_rt.entry_extrl,
            &instance_default_rt.instance_extrl,
        );
        let messenger_extrl = vk::DebugUtilsMessengerEXT::auto_assemble(&debug_utils_extrl)?;
        Ok(DebugMessengerDefaultRt {
            debug_utils_extrl,
            messenger_extrl,
            desc: "vulkan_debug_messenger",
        })
    }
}
