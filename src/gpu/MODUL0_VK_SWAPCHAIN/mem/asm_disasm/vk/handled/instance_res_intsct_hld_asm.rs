//! MCU **instance** — `ash::Instance` (validation + platform surface + debug).

use ash::vk;
use raw_window_handle::RawDisplayHandle;
use std::ffi::CString;

use crate::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::debug_msg_create_info::debug_msg_create_info;
use crate::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::instance::update_instance_layer_names;
use crate::{from_err, map_vk, ModulResult};

/// Catalog — create `ash::Instance`.
pub trait InstanceHandled {
    fn handled_assemble(
        entry_extrl: &ash::Entry,
        validation_layers_stp: bool,
        display_handle_extrl: RawDisplayHandle,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

impl InstanceHandled for ash::Instance {
    fn handled_assemble(
        entry_extrl: &ash::Entry,
        validation_layers_stp: bool,
        display_handle_extrl: RawDisplayHandle,
    ) -> ModulResult<Self> {
        {
            let app_name_extrl = from_err(CString::new("Modul"))?;
            let engine_name_extrl = from_err(CString::new("Modul Engine"))?;
            let app_info = vk::ApplicationInfo::default()
                .application_name(&app_name_extrl)
                .application_version(vk::make_api_version(0, 0, 1, 0))
                .engine_name(&engine_name_extrl)
                .engine_version(vk::make_api_version(0, 0, 1, 0))
                .api_version(vk::API_VERSION_1_3);

            #[expect(clippy::unwrap_used, reason = "static ASCII layer name")]
            let validation_layer_extrl =
                CString::new("VK_LAYER_KHRONOS_validation").unwrap();
            let layer_names_extrl =
                update_instance_layer_names(validation_layers_stp, &validation_layer_extrl);

            let mut extension_names_extrl: Vec<*const std::ffi::c_char> =
                ash_window::enumerate_required_extensions(display_handle_extrl)
                    .map_err(|e| format!("enumerate_required_extensions: {e}"))?
                    .to_vec();
            // extension peel prepared in proc-style helper without nested match:
            crate::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::instance::push_debug_extension(
                validation_layers_stp,
                &mut extension_names_extrl,
            );

            let mut debug_create_info = debug_msg_create_info();
            let mut create_info = vk::InstanceCreateInfo::default()
                .application_info(&app_info)
                .enabled_layer_names(&layer_names_extrl)
                .enabled_extension_names(&extension_names_extrl);

            create_info = crate::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::instance::maybe_push_debug_next(
                validation_layers_stp,
                create_info,
                &mut debug_create_info,
            );

            map_vk(unsafe { entry_extrl.create_instance(&create_info, None) })
        }
    }
}
