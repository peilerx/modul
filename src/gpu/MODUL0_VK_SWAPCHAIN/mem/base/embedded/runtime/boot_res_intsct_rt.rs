//! Embedded **boot** resource intersection (M.BASE_RES_INTSCT · FIX-120).
//! Entry · instance · debug messenger co-created on bootstrap.

/// Vulkan entry link bag.
pub struct EntryDefaultRt {
    /// External / raw Vulkan handle or host pointer field `entry_extrl` (`entry` peel).
    pub entry_extrl: ash::Entry,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Vulkan instance bag.
pub struct InstanceDefaultRt {
    /// External / raw Vulkan handle or host pointer field `instance_extrl` (`instance` peel).
    pub instance_extrl: ash::Instance,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Debug utils messenger bag.
pub struct DebugMessengerDefaultRt {
    /// External / raw Vulkan handle or host pointer field `debug_utils_extrl` (`debug_utils` peel).
    pub debug_utils_extrl: ash::ext::debug_utils::Instance,
    /// External / raw Vulkan handle or host pointer field `messenger_extrl` (`messenger` peel).
    pub messenger_extrl: ash::vk::DebugUtilsMessengerEXT,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
