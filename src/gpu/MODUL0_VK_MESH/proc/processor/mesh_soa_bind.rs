//! Bind compute color target (STORAGE_IMAGE) for `vkCmdDispatch`.

use ash::vk;
use ash::Device;

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::RendererDefaultRtCrg;
use crate::ModulResult;

/// Write soa color view + heat buffer into renderer set 0.
pub fn bind_soa_color_image(
    device_extrl: &Device,
    renderer: &RendererDefaultRtCrg,
    display: &DisplayDefaultRtCrg,
) -> ModulResult<()> {
    let Some(sets) = renderer.descriptor_sets_default_rt_pkg.as_ref() else {
        return Ok(());
    };
    let Some(&set) = sets.descriptor_sets_extrl.first() else {
        return Ok(());
    };
    if display.soa_color_view_extrl == vk::ImageView::null() {
        return Ok(());
    }
    let img = vk::DescriptorImageInfo::default()
        .image_view(display.soa_color_view_extrl)
        .image_layout(vk::ImageLayout::GENERAL);
    let write_img = vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(0)
        .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
        .image_info(std::slice::from_ref(&img));
    if display.soa_heat_buffer_extrl == vk::Buffer::null() {
        unsafe {
            device_extrl.update_descriptor_sets(std::slice::from_ref(&write_img), &[]);
        }
    } else {
        let buf = vk::DescriptorBufferInfo::default()
            .buffer(display.soa_heat_buffer_extrl)
            .offset(0)
            .range(display.soa_heat_bytes_rt);
        let write_buf = vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(1)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(std::slice::from_ref(&buf));
        unsafe {
            device_extrl.update_descriptor_sets(&[write_img, write_buf], &[]);
        }
    }
    Ok(())
}
