//! STORAGE + TRANSFER_SRC color image for compute present.

use ash::vk;
use ash::{Device, Instance};

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::DisplayDefaultRtCrg;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::asm_disasm::vk::handled::image_res_intsct_hld_asm::ImageResIntsctHandled;
use crate::pick_vk_memory_type_vram_then_host;
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

/// Allocate `float[n]` heat SoA on VRAM. GTT only if the discrete heap is too small.
pub fn update_soa_heat_buffer(
    device_extrl: &Device,
    instance_extrl: &Instance,
    physical_device_extrl: vk::PhysicalDevice,
    count: u32,
    display: &mut DisplayDefaultRtCrg,
) -> ModulResult<&'static str> {
    destroy_soa_heat_buffer(device_extrl, display);
    let n = vk::DeviceSize::from(count.max(1));
    let size = n.saturating_mul(4);
    let create_info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let buffer = map_vk(unsafe { device_extrl.create_buffer(&create_info, None) })?;
    let req = unsafe { device_extrl.get_buffer_memory_requirements(buffer) };
    let (memory_type_index, heap_tag) = pick_vk_memory_type_vram_then_host(
        instance_extrl,
        physical_device_extrl,
        req.memory_type_bits,
        req.size.max(size),
    )
    .ok_or_else(|| format!("soa heat: no heap ≥ {} bytes", req.size))?;
    let alloc = vk::MemoryAllocateInfo::default()
        .allocation_size(req.size)
        .memory_type_index(memory_type_index);
    let memory = map_vk(unsafe { device_extrl.allocate_memory(&alloc, None) })?;
    map_vk(unsafe { device_extrl.bind_buffer_memory(buffer, memory, 0) })?;
    display.soa_heat_buffer_extrl = buffer;
    display.soa_heat_memory_extrl = memory;
    display.soa_heat_bytes_rt = size;
    display.soa_heat_cleared_rt = false;
    Ok(heap_tag)
}

/// Zero heat SoA on the GPU and wait. DEVICE_LOCAL starts as garbage;
/// `NaN`/`stored<0` would punch holes (the chewed boot cube).
pub fn clear_soa_heat_buffer(
    device_extrl: &Device,
    queue_extrl: vk::Queue,
    command_pool_extrl: vk::CommandPool,
    display: &mut DisplayDefaultRtCrg,
) -> ModulResult<()> {
    if display.soa_heat_buffer_extrl == vk::Buffer::null() || display.soa_heat_bytes_rt < 4 {
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
    const CHUNK: vk::DeviceSize = 64 * 1024 * 1024;
    let mut off: vk::DeviceSize = 0;
    while off < display.soa_heat_bytes_rt {
        let mut len = (display.soa_heat_bytes_rt - off).min(CHUNK);
        len &= !3;
        if len == 0 {
            break;
        }
        unsafe {
            device_extrl.cmd_fill_buffer(cmd, display.soa_heat_buffer_extrl, off, len, 0);
        }
        off += len;
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

/// Free heat SoA.
pub fn destroy_soa_heat_buffer(device_extrl: &Device, display: &mut DisplayDefaultRtCrg) {
    if display.soa_heat_buffer_extrl != vk::Buffer::null() {
        unsafe {
            device_extrl.destroy_buffer(display.soa_heat_buffer_extrl, None);
            device_extrl.free_memory(display.soa_heat_memory_extrl, None);
        }
        display.soa_heat_buffer_extrl = vk::Buffer::null();
        display.soa_heat_memory_extrl = vk::DeviceMemory::null();
        display.soa_heat_bytes_rt = 0;
        display.soa_heat_cleared_rt = false;
    }
}
