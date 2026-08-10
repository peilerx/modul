//! MCU **debug_msg** — loader + messenger + create-info (local only · FIX-120).

use ash::ext::debug_utils;
use ash::vk;

use crate::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::debug_msg_create_info::debug_msg_create_info;
use crate::{map_vk, ModulResult};

/// Catalog — debug-messenger create-info.
pub trait DebugMsgCreateInfoAuto {
    fn auto_assemble() -> Self;
}

/// Catalog — debug utils instance loader.
pub trait DebugMsgLoaderAuto {
    fn auto_assemble(entry_extrl: &ash::Entry, instance_extrl: &ash::Instance) -> Self;
}

/// Catalog — debug utils messenger handle.
pub trait DebugMsgMessengerAuto {
    fn auto_assemble(debug_msg_extrl: &debug_utils::Instance) -> ModulResult<Self>
    where
        Self: Sized;
}

impl DebugMsgCreateInfoAuto for vk::DebugUtilsMessengerCreateInfoEXT<'static> {
    fn auto_assemble() -> Self {
        debug_msg_create_info()
    }
}

impl DebugMsgLoaderAuto for debug_utils::Instance {
    fn auto_assemble(entry_extrl: &ash::Entry, instance_extrl: &ash::Instance) -> Self {
        debug_utils::Instance::new(entry_extrl, instance_extrl)
    }
}

impl DebugMsgMessengerAuto for vk::DebugUtilsMessengerEXT {
    fn auto_assemble(debug_msg_extrl: &debug_utils::Instance) -> ModulResult<Self> {
        let debug_create_info = debug_msg_create_info();
        map_vk(unsafe {
            debug_msg_extrl.create_debug_utils_messenger(&debug_create_info, None)
        })
    }
}
