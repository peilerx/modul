//! vk brick: `vk::Framebuffer`.

use ash::vk;

use crate::{map_vk, ModulResult};

/// Catalog — framebuffer for render pass + attachments + extent.
pub trait FramebufferHandled {
    fn handled_assemble(
        device_extrl: &ash::Device,
        render_pass_extrl: vk::RenderPass,
        attachments_extrl: &[vk::ImageView],
        extent_stp: vk::Extent2D,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

impl FramebufferHandled for vk::Framebuffer {
    fn handled_assemble(
        device_extrl: &ash::Device,
        render_pass_extrl: vk::RenderPass,
        attachments_extrl: &[vk::ImageView],
        extent_stp: vk::Extent2D,
    ) -> ModulResult<Self> {
        let framebuffer_info_extrl = vk::FramebufferCreateInfo::default()
            .render_pass(render_pass_extrl)
            .attachments(attachments_extrl)
            .width(extent_stp.width)
            .height(extent_stp.height)
            .layers(1);
        map_vk(unsafe { device_extrl.create_framebuffer(&framebuffer_info_extrl, None) })
    }
}
