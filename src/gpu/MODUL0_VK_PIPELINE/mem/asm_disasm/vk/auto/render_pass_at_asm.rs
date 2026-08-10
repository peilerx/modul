//! vk brick: `vk::RenderPass` from attachment + subpass descriptions.

use ash::vk;
use ash::Device;

use crate::{map_vk, ModulResult};

/// Catalog — create render pass (flexible attachments / subpasses / dependencies).
pub trait RenderPassAuto {
    fn auto_assemble(
        device_extrl: &Device,
        attachments_extrl: &[vk::AttachmentDescription],
        subpasses_extrl: &[vk::SubpassDescription<'_>],
        dependencies_extrl: &[vk::SubpassDependency],
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

impl RenderPassAuto for vk::RenderPass {
    fn auto_assemble(
        device_extrl: &Device,
        attachments_extrl: &[vk::AttachmentDescription],
        subpasses_extrl: &[vk::SubpassDescription<'_>],
        dependencies_extrl: &[vk::SubpassDependency],
    ) -> ModulResult<Self> {
        let create_info = vk::RenderPassCreateInfo::default()
            .attachments(attachments_extrl)
            .subpasses(subpasses_extrl)
            .dependencies(dependencies_extrl);
        map_vk(unsafe { device_extrl.create_render_pass(&create_info, None) })
    }
}
