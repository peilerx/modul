//! vk brick: `vk::DescriptorSetLayout`.

use ash::vk;
use ash::Device;

use crate::{map_vk, ModulResult};

/// Catalog — descriptor set layout from bindings.
pub trait DescriptorSetLayoutAuto {
    fn auto_assemble(
        device_extrl: &Device,
        bindings_extrl: &[vk::DescriptorSetLayoutBinding<'_>],
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

impl DescriptorSetLayoutAuto for vk::DescriptorSetLayout {
    fn auto_assemble(
        device_extrl: &Device,
        bindings_extrl: &[vk::DescriptorSetLayoutBinding<'_>],
    ) -> ModulResult<Self> {
        let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(bindings_extrl);
        map_vk(unsafe { device_extrl.create_descriptor_set_layout(&create_info, None) })
    }
}
