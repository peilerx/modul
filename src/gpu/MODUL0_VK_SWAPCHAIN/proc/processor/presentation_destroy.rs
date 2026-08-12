//! Presentation GPU free (P · null checks · order FB → MSAA → depth → views → KHR).

use ash::khr::swapchain::Device as SwapchainDevice;
use ash::vk;
use ash::Device;

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::runtime::present_res_intsct_rt_pkgs::PresentationDefaultRtCrg;

/// Destroy framebuffer / MSAA / depth / views / KHR swapchain (caller: device idle).
pub fn destroy_presentation_gpu(
    device_extrl: &Device,
    swapchain_loader_extrl: &SwapchainDevice,
    presentation: &mut PresentationDefaultRtCrg,
) {
    unsafe {
        for fb in &presentation.framebuffer_default_rt_pkg.framebuffers_extrl {
            if *fb != vk::Framebuffer::null() {
                device_extrl.destroy_framebuffer(*fb, None);
            }
        }
        presentation
            .framebuffer_default_rt_pkg
            .framebuffers_extrl
            .clear();

        destroy_image_bundle(
            device_extrl,
            &mut presentation.msaa_color_default_rt_pkg.images_extrl,
            &mut presentation.msaa_color_default_rt_pkg.image_views_extrl,
            &mut presentation.msaa_color_default_rt_pkg.device_memories_extrl,
        );
        destroy_image_bundle(
            device_extrl,
            &mut presentation.depth_images_default_rt_pkg.images_extrl,
            &mut presentation.depth_images_default_rt_pkg.image_views_extrl,
            &mut presentation.depth_images_default_rt_pkg.device_memories_extrl,
        );

        for view in &presentation
            .swapchain_image_views_default_rt_pkg
            .image_views_extrl
        {
            if *view != vk::ImageView::null() {
                device_extrl.destroy_image_view(*view, None);
            }
        }
        presentation
            .swapchain_image_views_default_rt_pkg
            .image_views_extrl
            .clear();

        let sc = presentation.swapchain_default_rt_pkg.swapchain_extrl;
        if sc != vk::SwapchainKHR::null() {
            swapchain_loader_extrl.destroy_swapchain(sc, None);
        }
        presentation.swapchain_default_rt_pkg.swapchain_extrl = vk::SwapchainKHR::null();
        presentation.swapchain_default_rt_pkg.images_extrl.clear();
    }
}

fn destroy_image_bundle(
    device: &Device,
    images: &mut Vec<vk::Image>,
    views: &mut Vec<vk::ImageView>,
    memories: &mut Vec<vk::DeviceMemory>,
) {
    unsafe {
        for view in views.iter() {
            if *view != vk::ImageView::null() {
                device.destroy_image_view(*view, None);
            }
        }
        views.clear();
        for image in images.iter() {
            if *image != vk::Image::null() {
                device.destroy_image(*image, None);
            }
        }
        images.clear();
        for mem in memories.iter() {
            if *mem != vk::DeviceMemory::null() {
                device.free_memory(*mem, None);
            }
        }
        memories.clear();
    }
}
