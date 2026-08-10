//! vk brick: allocate `Vec<vk::DescriptorSet>`.

use ash::vk;
use ash::Device;

use crate::{map_vk, ModulResult};

/// Catalog — allocate descriptor sets from pool + layouts.
pub trait DescriptorSetsAllocateAuto {
    fn auto_assemble(
        device_extrl: &Device,
        descriptor_pool_extrl: vk::DescriptorPool,
        set_layouts_extrl: &[vk::DescriptorSetLayout],
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

impl DescriptorSetsAllocateAuto for Vec<vk::DescriptorSet> {
    fn auto_assemble(
        device_extrl: &Device,
        descriptor_pool_extrl: vk::DescriptorPool,
        set_layouts_extrl: &[vk::DescriptorSetLayout],
    ) -> ModulResult<Self> {
        let alloc_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(descriptor_pool_extrl)
            .set_layouts(set_layouts_extrl);
        map_vk(unsafe { device_extrl.allocate_descriptor_sets(&alloc_info) })
    }
}
