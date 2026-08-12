//! Processor module `gpu/MODUL0_VK_SWAPCHAIN/proc/processor`.
//!
//! Domain logic / record-draw helpers (PROTOCOL P).

pub mod debug_messenger;
/// Submodule `debug_msg_create_info`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/proc/processor` under the mem/conv/proc MCG canon.
pub mod debug_msg_create_info;
/// Device capability picks (MSAA · depth format).
pub mod device_caps;
/// Submodule `framebuffer`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/proc/processor` under the mem/conv/proc MCG canon.
pub mod framebuffer;
/// Submodule `instance`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/proc/processor` under the mem/conv/proc MCG canon.
pub mod instance;
/// Submodule `physical_device`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/proc/processor` under the mem/conv/proc MCG canon.
pub mod physical_device;
/// Presentation GPU free (called from Handled disassemble).
pub mod presentation_destroy;
/// Submodule `repeat_lane`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/proc/processor` under the mem/conv/proc MCG canon.
pub mod repeat_lane;
/// Submodule `swapchain`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/proc/processor` under the mem/conv/proc MCG canon.
pub mod swapchain;
