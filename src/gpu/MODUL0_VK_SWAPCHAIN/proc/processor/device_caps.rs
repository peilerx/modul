//! Physical-device capability picks for presentation (MSAA · depth).

use ash::vk;

use crate::ModulResult;

/// Prefer 4× MSAA when color∩depth both allow it; else 1×.
#[must_use]
pub fn pick_sample_count(instance: &ash::Instance, phys: vk::PhysicalDevice) -> vk::SampleCountFlags {
    pick_sample_count_prefer(
        instance,
        phys,
        SampleCountPrefer::Prefer4Else1,
    )
}

/// Preference for sample-count resolution (session / presentation knobs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SampleCountPrefer {
    #[default]
    Prefer4Else1,
    Force1,
    Prefer8Else4Else1,
}

/// Resolve sample count from device caps + preference.
#[must_use]
pub fn pick_sample_count_prefer(
    instance: &ash::Instance,
    phys: vk::PhysicalDevice,
    prefer: SampleCountPrefer,
) -> vk::SampleCountFlags {
    let props = unsafe { instance.get_physical_device_properties(phys) };
    let bits = props.limits.framebuffer_color_sample_counts
        & props.limits.framebuffer_depth_sample_counts;
    match prefer {
        SampleCountPrefer::Force1 => vk::SampleCountFlags::TYPE_1,
        SampleCountPrefer::Prefer4Else1 => {
            if bits.contains(vk::SampleCountFlags::TYPE_4) {
                vk::SampleCountFlags::TYPE_4
            } else {
                vk::SampleCountFlags::TYPE_1
            }
        }
        SampleCountPrefer::Prefer8Else4Else1 => {
            if bits.contains(vk::SampleCountFlags::TYPE_8) {
                vk::SampleCountFlags::TYPE_8
            } else if bits.contains(vk::SampleCountFlags::TYPE_4) {
                vk::SampleCountFlags::TYPE_4
            } else {
                vk::SampleCountFlags::TYPE_1
            }
        }
    }
}

/// Depth format candidates for product path (first supported wins): D32 → D24S8 → D16.
pub fn pick_depth_format(
    instance: &ash::Instance,
    phys: vk::PhysicalDevice,
) -> ModulResult<vk::Format> {
    const CANDIDATES: [vk::Format; 3] = [
        vk::Format::D32_SFLOAT,
        vk::Format::D24_UNORM_S8_UINT,
        vk::Format::D16_UNORM,
    ];
    for format in CANDIDATES {
        let props = unsafe { instance.get_physical_device_format_properties(phys, format) };
        if props
            .optimal_tiling_features
            .contains(vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT)
        {
            return Ok(format);
        }
    }
    Err("swapchain: no depth format (D32/D24/D16) with DEPTH_STENCIL_ATTACHMENT".into())
}
