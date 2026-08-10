//! MCU **swapchain_khr** — KHR create product as tuple (¬ pub type · M.GEN_NO_TYPE).

use ash::khr::surface::Instance as SurfaceLoader;
use ash::khr::swapchain::Device as SwapchainDevice;
use ash::vk;

use crate::gpu::MODUL0_VK_SWAPCHAIN::proc::processor::swapchain::{
    update_swapchain_extent, update_swapchain_image_count, update_swapchain_surface_format,
};
use crate::{map_vk, ModulResult};

/// Catalog — create KHR swapchain + images + resolved format/extent (no bag).
pub trait SwapchainKhrHandled {
    fn handled_assemble(
        surface_extrl: vk::SurfaceKHR,
        surface_loader_extrl: &SurfaceLoader,
        physical_device_extrl: vk::PhysicalDevice,
        swapchain_loader_extrl: &SwapchainDevice,
        extent_width_stp: u32,
        extent_height_stp: u32,
        surface_format_op: vk::Format,
        present_mode_op: vk::PresentModeKHR,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

impl SwapchainKhrHandled
    for (
        vk::SwapchainKHR,
        Vec<vk::Image>,
        vk::SurfaceFormatKHR,
        vk::Extent2D,
    )
{
    fn handled_assemble(
        surface_extrl: vk::SurfaceKHR,
        surface_loader_extrl: &SurfaceLoader,
        physical_device_extrl: vk::PhysicalDevice,
        swapchain_loader_extrl: &SwapchainDevice,
        extent_width_stp: u32,
        extent_height_stp: u32,
        surface_format_op: vk::Format,
        present_mode_op: vk::PresentModeKHR,
    ) -> ModulResult<Self> {
        let surface_capabilities_extrl = map_vk(unsafe {
            surface_loader_extrl.get_physical_device_surface_capabilities(
                physical_device_extrl,
                surface_extrl,
            )
        })?;
        let surface_formats_extrl = map_vk(unsafe {
            surface_loader_extrl
                .get_physical_device_surface_formats(physical_device_extrl, surface_extrl)
        })?;
        let present_modes_extrl = map_vk(unsafe {
            surface_loader_extrl.get_physical_device_surface_present_modes(
                physical_device_extrl,
                surface_extrl,
            )
        })?;

        let surface_format_op =
            update_swapchain_surface_format(surface_format_op, &surface_formats_extrl)?;
        let present_mode_op = match present_mode_op {
            vk::PresentModeKHR::MAILBOX
                if present_modes_extrl.contains(&vk::PresentModeKHR::MAILBOX) =>
            {
                vk::PresentModeKHR::MAILBOX
            }
            vk::PresentModeKHR::MAILBOX => vk::PresentModeKHR::FIFO,
            mode if present_modes_extrl.contains(&mode) => mode,
            _ => vk::PresentModeKHR::FIFO,
        };
        let extent_rt =
            update_swapchain_extent(&surface_capabilities_extrl, extent_width_stp, extent_height_stp);
        let image_count_stp = update_swapchain_image_count(&surface_capabilities_extrl);

        let swapchain_create_info_extrl = vk::SwapchainCreateInfoKHR::default()
            .surface(surface_extrl)
            .min_image_count(image_count_stp)
            .image_format(surface_format_op.format)
            .image_color_space(surface_format_op.color_space)
            .image_extent(extent_rt)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .pre_transform(surface_capabilities_extrl.current_transform)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode_op)
            .clipped(true);

        let swapchain_extrl = map_vk(unsafe {
            swapchain_loader_extrl.create_swapchain(&swapchain_create_info_extrl, None)
        })?;
        let images_extrl =
            map_vk(unsafe { swapchain_loader_extrl.get_swapchain_images(swapchain_extrl) })?;

        Ok((
            swapchain_extrl,
            images_extrl,
            surface_format_op,
            extent_rt,
        ))
    }
}
