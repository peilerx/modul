//! PortMatch Auto · `SwapchainPrt` → DirectVk format + present mode.

use ash::vk;

use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::prt::swapchain_prt::SwapchainPrt;
use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::setup::boot_res_intsct_stp_pkgs::SurfaceWindowStpPkg;

/// DirectVk peel: surface format + present mode + static desc tag.
#[must_use]
#[inline]
pub const fn swapchain_prt_format_present(
    present_intent: SwapchainPrt,
) -> (vk::Format, vk::PresentModeKHR, &'static str) {
    match present_intent {
        SwapchainPrt::SRGB_MAILBOX => (
            vk::Format::B8G8R8A8_SRGB,
            vk::PresentModeKHR::MAILBOX,
            "swapchain_srgb_mailbox",
        ),
        SwapchainPrt::SRGB_FIFO => (
            vk::Format::B8G8R8A8_SRGB,
            vk::PresentModeKHR::FIFO,
            "swapchain_srgb_fifo",
        ),
        SwapchainPrt::UNORM_MAILBOX => (
            vk::Format::B8G8R8A8_UNORM,
            vk::PresentModeKHR::MAILBOX,
            "swapchain_unorm_mailbox",
        ),
    }
}

/// Catalog — surface window setup bag from raw peels (base ¬ impl).
#[must_use]
#[inline]
pub const fn surface_window_stp_from_raw(
    display_handle_extrl: raw_window_handle::RawDisplayHandle,
    window_handle_extrl: raw_window_handle::RawWindowHandle,
    desc: &'static str,
) -> SurfaceWindowStpPkg {
    SurfaceWindowStpPkg {
        display_handle_extrl,
        window_handle_extrl,
        desc,
    }
}
