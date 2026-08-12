//! MCU buffer + host-visible device memory for mesh VBO/IBO.

use ash::vk;
use ash::{Device, Instance};

use crate::{find_vk_memory_type, map_vk, ModulResult};

/// Host-visible buffer bind (VERTEX / INDEX).
/// Caller supplies `size_stp` already ≥ 4 when non-empty.
pub trait BufferHostVisibleHandled {
    fn handled_assemble(
        device_extrl: &Device,
        instance_extrl: &Instance,
        physical_device_extrl: vk::PhysicalDevice,
        size_stp: vk::DeviceSize,
        usage_op: vk::BufferUsageFlags,
    ) -> ModulResult<(vk::Buffer, vk::DeviceMemory)>;
}

impl BufferHostVisibleHandled for (vk::Buffer, vk::DeviceMemory) {
    fn handled_assemble(
        device_extrl: &Device,
        instance_extrl: &Instance,
        physical_device_extrl: vk::PhysicalDevice,
        size_stp: vk::DeviceSize,
        usage_op: vk::BufferUsageFlags,
    ) -> ModulResult<(vk::Buffer, vk::DeviceMemory)> {
        {
            let create_info = vk::BufferCreateInfo::default()
                .size(size_stp)
                .usage(usage_op)
                .sharing_mode(vk::SharingMode::EXCLUSIVE);
            let buffer_extrl =
                map_vk(unsafe { device_extrl.create_buffer(&create_info, None) })?;
            let req = unsafe { device_extrl.get_buffer_memory_requirements(buffer_extrl) };
            let memory_type_index_stp = find_vk_memory_type(
                instance_extrl,
                physical_device_extrl,
                req.memory_type_bits,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            )
            .ok_or_else(|| "mesh buffer: no HOST_VISIBLE memory type".to_string())?;
            let alloc = vk::MemoryAllocateInfo::default()
                .allocation_size(req.size)
                .memory_type_index(memory_type_index_stp);
            let memory_extrl =
                map_vk(unsafe { device_extrl.allocate_memory(&alloc, None) })?;
            map_vk(unsafe {
                device_extrl.bind_buffer_memory(buffer_extrl, memory_extrl, 0)
            })?;
            Ok((buffer_extrl, memory_extrl))
        }
    }
}

/// Map host-visible memory and copy bytes.
pub fn handled_upload_host_visible(
    device_extrl: &Device,
    memory_extrl: vk::DeviceMemory,
    bytes_extrl: &[u8],
) -> ModulResult<()> {
    let n_stp = bytes_extrl.len();
    match n_stp {
        0 => Ok(()),
        n_stp => {
            let ptr = map_vk(unsafe {
                device_extrl.map_memory(
                    memory_extrl,
                    0,
                    n_stp as vk::DeviceSize,
                    vk::MemoryMapFlags::empty(),
                )
            })?;
            unsafe {
                std::ptr::copy_nonoverlapping(bytes_extrl.as_ptr(), ptr.cast::<u8>(), n_stp);
                device_extrl.unmap_memory(memory_extrl);
            }
            Ok(())
        }
    }
}
