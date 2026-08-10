//! Instance extension / pNext peels (P · no handle create).

use ash::ext::debug_utils;
use ash::vk;
use std::ffi::CString;

/// `update_instance_layer_names` — function (update instance layer names).
/// Public API entry for this module.
/// Belongs to: swapchain / device bootstrap MCG.
pub(crate) fn update_instance_layer_names(
    validation_layers_stp: bool,
    validation_layer_extrl: &CString,
) -> Vec<*const i8> {
    if validation_layers_stp {
        vec![validation_layer_extrl.as_ptr()]
    } else {
        Vec::new()
    }
}

/// `push_debug_extension` — function (push debug extension).
/// Public API entry for this module.
/// Belongs to: swapchain / device bootstrap MCG.
pub fn push_debug_extension(
    validation_layers_stp: bool,
    extension_names_extrl: &mut Vec<*const std::ffi::c_char>,
) {
    if validation_layers_stp {
        extension_names_extrl.push(debug_utils::NAME.as_ptr());
    }
}

/// `maybe_push_debug_next` — function (maybe push debug next).
/// Public API entry for this module.
/// Belongs to: swapchain / device bootstrap MCG.
pub fn maybe_push_debug_next<'a>(
    validation_layers_stp: bool,
    create_info: vk::InstanceCreateInfo<'a>,
    debug_create_info: &'a mut vk::DebugUtilsMessengerCreateInfoEXT<'static>,
) -> vk::InstanceCreateInfo<'a> {
    if validation_layers_stp {
        create_info.push_next(debug_create_info)
    } else {
        create_info
    }
}
