//! vk resource: `vk::CommandPool`.

use ash::vk;

use crate::{map_vk, ModulResult};

/// Catalog — command pool (reset flag) for a queue family.
pub trait CommandPoolHandled {
    fn handled_assemble(
        device_extrl: &ash::Device,
        queue_family_index_stp: u32,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

impl CommandPoolHandled for vk::CommandPool {
    fn handled_assemble(
        device_extrl: &ash::Device,
        queue_family_index_stp: u32,
    ) -> ModulResult<Self> {
        let create_info_extrl = vk::CommandPoolCreateInfo::default()
            .queue_family_index(queue_family_index_stp)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        map_vk(unsafe { device_extrl.create_command_pool(&create_info_extrl, None) })
    }
}
