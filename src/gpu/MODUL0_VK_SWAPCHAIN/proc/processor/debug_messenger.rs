use ash::vk;
use std::ffi::CStr;
use std::io::Write;
use std::sync::Mutex;

/// Optional path: when set, validation lines are also appended here (ship cubes log).
static VK_LOG_FILE: Mutex<Option<std::fs::File>> = Mutex::new(None);

/// Open/replace the file sink used by the debug messenger (best-effort).
pub fn set_vk_validation_log_path(path: &std::path::Path) {
    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        Ok(f) => {
            if let Ok(mut g) = VK_LOG_FILE.lock() {
                *g = Some(f);
            }
        }
        Err(e) => {
            let mut stderr = std::io::stderr().lock();
            let _ = writeln!(
                stderr,
                "[Vulkan] cannot open validation log {}: {e}",
                path.display()
            );
        }
    }
}

fn append_vk_log(line: &str) {
    if let Ok(mut g) = VK_LOG_FILE.lock() {
        if let Some(f) = g.as_mut() {
            let _ = writeln!(f, "{line}");
            let _ = f.flush();
        }
    }
}

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
        let line = format!("[Vulkan Error] {message}");
        let _ = writeln!(stderr, "{line}");
        append_vk_log(&line);
    } else if severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING) {
        let line = format!("[Vulkan Warning] {message}");
        let _ = writeln!(stderr, "{line}");
        append_vk_log(&line);
    } else if severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::INFO) {
        // Spec / performance info — file only (avoid stderr spam for ship testers).
        append_vk_log(&format!("[Vulkan Info] {message}"));
    } else if severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE) {
        append_vk_log(&format!("[Vulkan Verbose] {message}"));
    }

    vk::FALSE
}