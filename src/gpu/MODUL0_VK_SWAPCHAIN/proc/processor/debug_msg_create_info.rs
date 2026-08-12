//! Debug messenger create-info peel (P · shared by instance + debug messenger).

use ash::vk;

use crate::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::debug_messenger::update_vulkan_debug_callback;

/// Shared create-info for validation messenger (instance pNext + create messenger).
pub fn debug_msg_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT<'static> {
    vk::DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
            // Ship cubes writes ERROR/WARNING to stderr + file; INFO/VERBOSE to file only.
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(update_vulkan_debug_callback))
}
