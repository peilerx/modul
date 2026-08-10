//! vk brick: `vk::DescriptorPool`.

use ash::vk;
use ash::Device;

use crate::{map_vk, ModulResult};

/// Catalog — descriptor pool from max sets + pool sizes.
pub trait DescriptorPoolHandled {
    fn handled_assemble(
        device_extrl: &Device,
        max_sets_stp: u32,
        pool_sizes_extrl: &[vk::DescriptorPoolSize],
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

impl DescriptorPoolHandled for vk::DescriptorPool {
    fn handled_assemble(
        device_extrl: &Device,
        max_sets_stp: u32,
        pool_sizes_extrl: &[vk::DescriptorPoolSize],
    ) -> ModulResult<Self> {
        let create_info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(max_sets_stp)
            .pool_sizes(pool_sizes_extrl);
        map_vk(unsafe { device_extrl.create_descriptor_pool(&create_info, None) })
    }
}
