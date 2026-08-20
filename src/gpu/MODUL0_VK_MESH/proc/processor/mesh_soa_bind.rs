//! Bind compute color target (STORAGE_IMAGE) for `vkCmdDispatch`.

use ash::vk;
use ash::Device;

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::RendererDefaultRtCrg;
use crate::ModulResult;

/// Write soa color view + heat 3D view into renderer set 0.
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
    let color = vk::DescriptorImageInfo::default()
        .image_view(display.soa_color_view_extrl)
        .image_layout(vk::ImageLayout::GENERAL);
    let write_color = vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(0)
        .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
        .image_info(std::slice::from_ref(&color));
    if display.soa_heat_view_extrl == vk::ImageView::null() {
        unsafe {
            device_extrl.update_descriptor_sets(std::slice::from_ref(&write_color), &[]);
        }
    } else {
        let heat = vk::DescriptorImageInfo::default()
            .image_view(display.soa_heat_view_extrl)
            .image_layout(vk::ImageLayout::GENERAL);
        let write_heat = vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(1)
            .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
            .image_info(std::slice::from_ref(&heat));
        unsafe {
            device_extrl.update_descriptor_sets(&[write_color, write_heat], &[]);
        }
    }
    Ok(())
}
