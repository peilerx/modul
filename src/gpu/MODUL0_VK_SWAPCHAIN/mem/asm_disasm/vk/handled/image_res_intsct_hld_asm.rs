//! MCU **image_res_intsct** — resource intersection (N.RES_INTSCT · FIX-120).
//!
//! Layout (**N.FREQ** · hot → cold):
//!   1) all **traits** by hottest external use
//!   2) all **impls** in the **same** order

use ash::vk;

use crate::{find_vk_memory_type, map_vk, ModulResult};

// ── Traits (hot → cold) ─────────────────────────────────────────────────────

/// Catalog — full intersection: image → memory → bind → view (hottest pack API).
pub trait ImageResIntsctHandled {
    fn handled_assemble(
        device_extrl: &ash::Device,
        instance_extrl: &ash::Instance,
        physical_device_extrl: vk::PhysicalDevice,
        format_op: vk::Format,
        extent_stp: vk::Extent2D,
        sample_count_op: vk::SampleCountFlags,
        usage_op: vk::ImageUsageFlags,
        aspect_mask_op: vk::ImageAspectFlags,
        memory_properties_op: vk::MemoryPropertyFlags,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Catalog — 2D color image view (default aspect COLOR).
pub trait ImageViewHandled {
    fn handled_assemble(
        device_extrl: &ash::Device,
        image_extrl: vk::Image,
        format_op: vk::Format,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Catalog — 2D image view (aspect · format knobs).
pub trait ImageViewAspectHandled {
    fn handled_assemble(
        device_extrl: &ash::Device,
        image_extrl: vk::Image,
        format_op: vk::Format,
        aspect_mask_op: vk::ImageAspectFlags,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Catalog — 2D image (format · extent · samples · usage).
pub trait ImageHandled {
    fn handled_assemble(
        device_extrl: &ash::Device,
        format_op: vk::Format,
        extent_stp: vk::Extent2D,
        sample_count_op: vk::SampleCountFlags,
        usage_op: vk::ImageUsageFlags,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Catalog — allocate memory matching an image + property flags.
pub trait DeviceMemoryImageHandled {
    fn handled_assemble(
        device_extrl: &ash::Device,
        instance_extrl: &ash::Instance,
        physical_device_extrl: vk::PhysicalDevice,
        image_extrl: vk::Image,
        memory_properties_op: vk::MemoryPropertyFlags,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Catalog — raw allocate by size + type index.
pub trait DeviceMemoryHandled {
    fn handled_assemble(
        device_extrl: &ash::Device,
        allocation_size_stp: vk::DeviceSize,
        memory_type_index_stp: u32,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Catalog — bind image ↔ memory (no new handle).
pub trait ImageMemoryBindHandled {
    fn handled_assemble(
        device_extrl: &ash::Device,
        image_extrl: vk::Image,
        memory_extrl: vk::DeviceMemory,
        memory_offset_stp: vk::DeviceSize,
    ) -> ModulResult<()>;
}

// ── Impls (same order) ──────────────────────────────────────────────────────

impl ImageResIntsctHandled for (vk::Image, vk::DeviceMemory, vk::ImageView) {
    fn handled_assemble(
        device_extrl: &ash::Device,
        instance_extrl: &ash::Instance,
        physical_device_extrl: vk::PhysicalDevice,
        format_op: vk::Format,
        extent_stp: vk::Extent2D,
        sample_count_op: vk::SampleCountFlags,
        usage_op: vk::ImageUsageFlags,
        aspect_mask_op: vk::ImageAspectFlags,
        memory_properties_op: vk::MemoryPropertyFlags,
    ) -> ModulResult<Self> {
        match format_op {
            format_op => {
                let image_extrl = vk::Image::handled_assemble(
                    device_extrl,
                    format_op,
                    extent_stp,
                    sample_count_op,
                    usage_op,
                )?;
                let memory_extrl =
                    <vk::DeviceMemory as DeviceMemoryImageHandled>::handled_assemble(
                        device_extrl,
                        instance_extrl,
                        physical_device_extrl,
                        image_extrl,
                        memory_properties_op,
                    )?;
                <() as ImageMemoryBindHandled>::handled_assemble(
                    device_extrl,
                    image_extrl,
                    memory_extrl,
                    0,
                )?;
                let view_extrl = <vk::ImageView as ImageViewAspectHandled>::handled_assemble(
                    device_extrl,
                    image_extrl,
                    format_op,
                    aspect_mask_op,
                )?;
                Ok((image_extrl, memory_extrl, view_extrl))
            }
        }
    }
}

impl ImageViewHandled for vk::ImageView {
    fn handled_assemble(
        device_extrl: &ash::Device,
        image_extrl: vk::Image,
        format_op: vk::Format,
    ) -> ModulResult<Self> {
        match format_op {
            format_op => <Self as ImageViewAspectHandled>::handled_assemble(
                device_extrl,
                image_extrl,
                format_op,
                vk::ImageAspectFlags::COLOR,
            ),
        }
    }
}

impl ImageViewAspectHandled for vk::ImageView {
    fn handled_assemble(
        device_extrl: &ash::Device,
        image_extrl: vk::Image,
        format_op: vk::Format,
        aspect_mask_op: vk::ImageAspectFlags,
    ) -> ModulResult<Self> {
        match format_op {
            format_op => {
                let image_view_create_info = vk::ImageViewCreateInfo::default()
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(format_op)
                    .components(vk::ComponentMapping {
                        r: vk::ComponentSwizzle::IDENTITY,
                        g: vk::ComponentSwizzle::IDENTITY,
                        b: vk::ComponentSwizzle::IDENTITY,
                        a: vk::ComponentSwizzle::IDENTITY,
                    })
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: aspect_mask_op,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    })
                    .image(image_extrl);
                map_vk(unsafe { device_extrl.create_image_view(&image_view_create_info, None) })
            }
        }
    }
}

impl ImageHandled for vk::Image {
    fn handled_assemble(
        device_extrl: &ash::Device,
        format_op: vk::Format,
        extent_stp: vk::Extent2D,
        sample_count_op: vk::SampleCountFlags,
        usage_op: vk::ImageUsageFlags,
    ) -> ModulResult<Self> {
        match format_op {
            format_op => {
                let image_info = vk::ImageCreateInfo::default()
                    .image_type(vk::ImageType::TYPE_2D)
                    .format(format_op)
                    .extent(vk::Extent3D {
                        width: extent_stp.width,
                        height: extent_stp.height,
                        depth: 1,
                    })
                    .mip_levels(1)
                    .array_layers(1)
                    .samples(sample_count_op)
                    .tiling(vk::ImageTiling::OPTIMAL)
                    .usage(usage_op)
                    .sharing_mode(vk::SharingMode::EXCLUSIVE)
                    .initial_layout(vk::ImageLayout::UNDEFINED);
                map_vk(unsafe { device_extrl.create_image(&image_info, None) })
            }
        }
    }
}

impl DeviceMemoryImageHandled for vk::DeviceMemory {
    fn handled_assemble(
        device_extrl: &ash::Device,
        instance_extrl: &ash::Instance,
        physical_device_extrl: vk::PhysicalDevice,
        image_extrl: vk::Image,
        memory_properties_op: vk::MemoryPropertyFlags,
    ) -> ModulResult<Self> {
        match memory_properties_op {
            memory_properties_op => {
                let mem_requirements_extrl =
                    unsafe { device_extrl.get_image_memory_requirements(image_extrl) };
                let memory_type_index_stp = find_vk_memory_type(
                    instance_extrl,
                    physical_device_extrl,
                    mem_requirements_extrl.memory_type_bits,
                    memory_properties_op,
                )
                .ok_or_else(|| "Failed to find memory type for image".to_string())?;
                <Self as DeviceMemoryHandled>::handled_assemble(
                    device_extrl,
                    mem_requirements_extrl.size,
                    memory_type_index_stp,
                )
            }
        }
    }
}

impl DeviceMemoryHandled for vk::DeviceMemory {
    fn handled_assemble(
        device_extrl: &ash::Device,
        allocation_size_stp: vk::DeviceSize,
        memory_type_index_stp: u32,
    ) -> ModulResult<Self> {
        let alloc_info = vk::MemoryAllocateInfo::default()
            .allocation_size(allocation_size_stp)
            .memory_type_index(memory_type_index_stp);
        map_vk(unsafe { device_extrl.allocate_memory(&alloc_info, None) })
    }
}

impl ImageMemoryBindHandled for () {
    fn handled_assemble(
        device_extrl: &ash::Device,
        image_extrl: vk::Image,
        memory_extrl: vk::DeviceMemory,
        memory_offset_stp: vk::DeviceSize,
    ) -> ModulResult<()> {
        map_vk(unsafe {
            device_extrl.bind_image_memory(image_extrl, memory_extrl, memory_offset_stp)
        })
    }
}
