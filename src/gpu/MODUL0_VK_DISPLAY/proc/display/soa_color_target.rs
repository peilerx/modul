//! STORAGE + TRANSFER_SRC color image for compute present.

use ash::vk;
use ash::{Device, Instance};

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::handled::image_res_intsct_hld_asm::{
    Image3dResIntsctHandled, ImageResIntsctHandled,
};
use crate::{map_vk, ModulResult};

/// 1× target. AA is coverage in the SoA compute shader, not extra pixels.
pub const SOA_SSAA: u32 = 1;

/// Allocate the compute color target at swapchain extent × `SOA_SSAA`.
pub fn update_soa_color_target(
    device_extrl: &Device,
    instance_extrl: &Instance,
    physical_device_extrl: vk::PhysicalDevice,
    swap_extent: vk::Extent2D,
    display: &mut DisplayDefaultRtCrg,
) -> ModulResult<()> {
    destroy_soa_color_target(device_extrl, display);
    let extent = vk::Extent2D {
        width: swap_extent.width.saturating_mul(SOA_SSAA).max(1),
        height: swap_extent.height.saturating_mul(SOA_SSAA).max(1),
    };
    let (image, memory, view) =
        <(vk::Image, vk::DeviceMemory, vk::ImageView) as ImageResIntsctHandled>::handled_assemble(
            device_extrl,
            instance_extrl,
            physical_device_extrl,
            vk::Format::R8G8B8A8_UNORM,
            extent,
            vk::SampleCountFlags::TYPE_1,
            vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::TRANSFER_SRC,
            vk::ImageAspectFlags::COLOR,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;
    display.soa_color_image_extrl = image;
    display.soa_color_memory_extrl = memory;
    display.soa_color_view_extrl = view;
    display.soa_color_extent_rt = extent;
    Ok(())
}

/// Free compute color target.
pub fn destroy_soa_color_target(device_extrl: &Device, display: &mut DisplayDefaultRtCrg) {
    if display.soa_color_view_extrl != vk::ImageView::null() {
        unsafe {
            device_extrl.destroy_image_view(display.soa_color_view_extrl, None);
        }
        display.soa_color_view_extrl = vk::ImageView::null();
    }
    if display.soa_color_image_extrl != vk::Image::null() {
        unsafe {
            device_extrl.destroy_image(display.soa_color_image_extrl, None);
            device_extrl.free_memory(display.soa_color_memory_extrl, None);
        }
        display.soa_color_image_extrl = vk::Image::null();
        display.soa_color_memory_extrl = vk::DeviceMemory::null();
    }
    display.soa_color_extent_rt = vk::Extent2D::default();
}

/// Allocate heat SoA as `VkImage` TYPE_3D R32F (one channel = one field).
pub fn update_soa_heat_image(
    device_extrl: &Device,
    instance_extrl: &Instance,
    physical_device_extrl: vk::PhysicalDevice,
    count: u32,
    display: &mut DisplayDefaultRtCrg,
) -> ModulResult<&'static str> {
    destroy_soa_heat_image(device_extrl, display);
    let nx = f64::from(count.max(1)).cbrt().round().max(1.0) as u32;
    let extent = vk::Extent3D {
        width: nx,
        height: nx,
        depth: nx,
    };
    let (image, memory, view) =
        <(vk::Image, vk::DeviceMemory, vk::ImageView) as Image3dResIntsctHandled>::handled_assemble(
            device_extrl,
            instance_extrl,
            physical_device_extrl,
            vk::Format::R32_SFLOAT,
            extent,
            vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::TRANSFER_DST,
            vk::ImageAspectFlags::COLOR,
        )?;
    let req = unsafe { device_extrl.get_image_memory_requirements(image) };
    display.soa_heat_image_extrl = image;
    display.soa_heat_view_extrl = view;
    display.soa_heat_memory_extrl = memory;
    display.soa_heat_extent_rt = extent;
    display.soa_heat_bytes_rt = req.size;
    display.soa_heat_cleared_rt = false;
    Ok("TYPE_3D R32F VRAM")
}

/// Zero the 3D heat volume (`vkCmdClearColorImage`) and wait.
pub fn clear_soa_heat_image(
    device_extrl: &Device,
    queue_extrl: vk::Queue,
    command_pool_extrl: vk::CommandPool,
    display: &mut DisplayDefaultRtCrg,
) -> ModulResult<()> {
    if display.soa_heat_image_extrl == vk::Image::null() {
        display.soa_heat_cleared_rt = true;
        return Ok(());
    }
    let alloc_info = vk::CommandBufferAllocateInfo::default()
        .command_pool(command_pool_extrl)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);
    let cbs = map_vk(unsafe { device_extrl.allocate_command_buffers(&alloc_info) })?;
    let cmd = cbs[0];
    let begin = vk::CommandBufferBeginInfo::default()
        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    map_vk(unsafe { device_extrl.begin_command_buffer(cmd, &begin) })?;
    let range = vk::ImageSubresourceRange {
        aspect_mask: vk::ImageAspectFlags::COLOR,
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    };
    let to_dst = vk::ImageMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::empty())
        .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
        .old_layout(vk::ImageLayout::UNDEFINED)
        .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(display.soa_heat_image_extrl)
        .subresource_range(range);
    unsafe {
        device_extrl.cmd_pipeline_barrier(
            cmd,
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            std::slice::from_ref(&to_dst),
        );
        let clear = vk::ClearColorValue {
            float32: [0.0, 0.0, 0.0, 0.0],
        };
        device_extrl.cmd_clear_color_image(
            cmd,
            display.soa_heat_image_extrl,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &clear,
            std::slice::from_ref(&range),
        );
    }
    let to_general = vk::ImageMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
        .dst_access_mask(vk::AccessFlags::SHADER_READ | vk::AccessFlags::SHADER_WRITE)
        .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .new_layout(vk::ImageLayout::GENERAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(display.soa_heat_image_extrl)
        .subresource_range(range);
    unsafe {
        device_extrl.cmd_pipeline_barrier(
            cmd,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::COMPUTE_SHADER,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            std::slice::from_ref(&to_general),
        );
    }
    map_vk(unsafe { device_extrl.end_command_buffer(cmd) })?;
    let cmd_arr = [cmd];
    let submits = [vk::SubmitInfo::default().command_buffers(&cmd_arr)];
    unsafe {
        map_vk(device_extrl.queue_submit(queue_extrl, &submits, vk::Fence::null()))?;
        map_vk(device_extrl.queue_wait_idle(queue_extrl))?;
        device_extrl.free_command_buffers(command_pool_extrl, &cmd_arr);
    }
    display.soa_heat_cleared_rt = true;
    Ok(())
}

/// Free heat volume.
pub fn destroy_soa_heat_image(device_extrl: &Device, display: &mut DisplayDefaultRtCrg) {
    if display.soa_heat_view_extrl != vk::ImageView::null() {
        unsafe {
            device_extrl.destroy_image_view(display.soa_heat_view_extrl, None);
        }
        display.soa_heat_view_extrl = vk::ImageView::null();
    }
    if display.soa_heat_image_extrl != vk::Image::null() {
        unsafe {
            device_extrl.destroy_image(display.soa_heat_image_extrl, None);
            device_extrl.free_memory(display.soa_heat_memory_extrl, None);
        }
        display.soa_heat_image_extrl = vk::Image::null();
        display.soa_heat_memory_extrl = vk::DeviceMemory::null();
        display.soa_heat_extent_rt = vk::Extent3D::default();
        display.soa_heat_bytes_rt = 0;
        display.soa_heat_cleared_rt = false;
    }
}
