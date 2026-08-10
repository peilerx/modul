//! Base **present** resource intersection (M.BASE_RES_INTSCT · N.RES_INTSCT · FIX-120).
//!
//! All *Pkg that field each other live **in this file only** · ¬ import sibling res_intsct bags.
//! Presentation lane: swapchain KHR · views · depth · msaa · sample · framebuffer · crg.

/// KHR swapchain — runtime handles only (FIX-086).
pub struct SwapchainDefaultRtPkg {
    /// Operator / knob field `surface_format_op`.
    pub surface_format_op: ash::vk::SurfaceFormatKHR,
    /// Runtime phase field `extent_rt`.
    pub extent_rt: ash::vk::Extent2D,
    /// External / raw Vulkan handle or host pointer field `swapchain_extrl` (`swapchain` peel).
    pub swapchain_extrl: ash::vk::SwapchainKHR,
    /// External / raw Vulkan handle or host pointer field `images_extrl` (`images` peel).
    pub images_extrl: Vec<ash::vk::Image>,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Color image views for swapchain images.
pub struct SwapchainImageViewsDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `image_views_extrl` (`image_views` peel).
    pub image_views_extrl: Vec<ash::vk::ImageView>,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Depth attachments: image + view + memory per swapchain slot.
pub struct DepthImagesDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `images_extrl` (`images` peel).
    pub images_extrl: Vec<ash::vk::Image>,
    /// External / raw Vulkan handle or host pointer field `image_views_extrl` (`image_views` peel).
    pub image_views_extrl: Vec<ash::vk::ImageView>,
    /// External / raw Vulkan handle or host pointer field `device_memories_extrl` (`device_memories` peel).
    pub device_memories_extrl: Vec<ash::vk::DeviceMemory>,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// MSAA color attachments: image + view + memory per slot (empty when sample count = 1).
pub struct MsaaColorDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `images_extrl` (`images` peel).
    pub images_extrl: Vec<ash::vk::Image>,
    /// External / raw Vulkan handle or host pointer field `image_views_extrl` (`image_views` peel).
    pub image_views_extrl: Vec<ash::vk::ImageView>,
    /// External / raw Vulkan handle or host pointer field `device_memories_extrl` (`device_memories` peel).
    pub device_memories_extrl: Vec<ash::vk::DeviceMemory>,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Sample count flags — runtime state only (FIX-086).
pub struct SampleCountDefaultRtPkg {
    /// Operator / knob field `sample_count_op`.
    pub sample_count_op: ash::vk::SampleCountFlags,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Framebuffers for presentation.
pub struct FramebufferDefaultRtPkg {
    /// External / raw Vulkan handle or host pointer field `framebuffers_extrl` (`framebuffers` peel).
    pub framebuffers_extrl: Vec<ash::vk::Framebuffer>,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Conv-assembled presentation CG export (runtime capstone).
pub struct PresentationDefaultRtCrg {
    /// Nested package bag field `swapchain_default_rt_pkg`.
    pub swapchain_default_rt_pkg: SwapchainDefaultRtPkg,
    /// Nested package bag field `swapchain_image_views_default_rt_pkg`.
    pub swapchain_image_views_default_rt_pkg: SwapchainImageViewsDefaultRtPkg,
    /// Nested package bag field `sample_count_default_rt_pkg`.
    pub sample_count_default_rt_pkg: SampleCountDefaultRtPkg,
    /// Nested package bag field `depth_images_default_rt_pkg`.
    pub depth_images_default_rt_pkg: DepthImagesDefaultRtPkg,
    /// Nested package bag field `msaa_color_default_rt_pkg`.
    pub msaa_color_default_rt_pkg: MsaaColorDefaultRtPkg,
    /// Nested package bag field `framebuffer_default_rt_pkg`.
    pub framebuffer_default_rt_pkg: FramebufferDefaultRtPkg,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
