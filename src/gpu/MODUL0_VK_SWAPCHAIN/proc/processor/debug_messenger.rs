use ash::vk;
use std::ffi::CStr;
use std::io::Write;

/// `update_vulkan_debug_callback` — function (update vulkan debug callback).
/// Public API entry for this module.
/// Belongs to: swapchain / device bootstrap MCG.
pub(crate) unsafe extern "system" fn update_vulkan_debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message = unsafe { CStr::from_ptr((*data).p_message) }.to_string_lossy();
    let mut stderr = std::io::stderr().lock();

    if severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR) {
        let _ = writeln!(stderr, "[Vulkan Error] {message}");
    } else if severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING) {
        let _ = writeln!(stderr, "[Vulkan Warning] {message}");
    }

    vk::FALSE
}