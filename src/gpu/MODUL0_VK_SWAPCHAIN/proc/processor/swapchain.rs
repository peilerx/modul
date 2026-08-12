use ash::vk;

use crate::ModulResult;

/// `update_swapchain_surface_format` — function (update swapchain surface format).
/// Public API entry for this module.
/// Belongs to: swapchain / device bootstrap MCG.
pub(crate) fn update_swapchain_surface_format(
    surface_format_op: vk::Format,
    surface_formats_extrl: &[vk::SurfaceFormatKHR],
) -> ModulResult<vk::SurfaceFormatKHR> {
    if surface_formats_extrl.is_empty() {
        return Err("swapchain: surface reported zero formats".into());
    }
    if let Some(found) = surface_formats_extrl.iter().find(|f| {
        f.format == surface_format_op && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
    }) {
        return Ok(*found);
    }
    if let Some(found) = surface_formats_extrl
        .iter()
        .find(|f| f.format == surface_format_op)
    {
        return Ok(*found);
    }
    // Prefer any remaining entry after format filters failed.
    surface_formats_extrl
        .first()
        .copied()
        .ok_or_else(|| "swapchain: surface reported zero formats".into())
}

/// `update_swapchain_extent` — function (update swapchain extent).
/// Public API entry for this module.
/// Belongs to: swapchain / device bootstrap MCG.
pub(crate) const fn update_swapchain_extent(
    capabilities_extrl: &vk::SurfaceCapabilitiesKHR,
    extent_width_stp: u32,
    extent_height_stp: u32,
) -> vk::Extent2D {
    if capabilities_extrl.current_extent.width == u32::MAX {
        vk::Extent2D {
            width: extent_width_stp,
            height: extent_height_stp,
        }
    } else {
        capabilities_extrl.current_extent
    }
}

/// `update_swapchain_image_count` — function (update swapchain image count).
/// Public API entry for this module.
/// Belongs to: swapchain / device bootstrap MCG.
pub(crate) const fn update_swapchain_image_count(
    capabilities_extrl: &vk::SurfaceCapabilitiesKHR,
) -> u32 {
    #[expect(clippy::arithmetic_side_effects, reason = "standard swapchain image count sizing")]
    let mut image_count_stp = capabilities_extrl.min_image_count + 1;
    if capabilities_extrl.max_image_count > 0 && image_count_stp > capabilities_extrl.max_image_count
    {
        image_count_stp = capabilities_extrl.max_image_count;
    }
    image_count_stp
}