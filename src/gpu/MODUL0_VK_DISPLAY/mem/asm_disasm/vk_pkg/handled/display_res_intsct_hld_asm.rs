//! MCU **display_res_intsct** — command · render · vulkan display (N.RES_INTSCT · N.FREQ · FIX-120).
//! Mixed Auto/Handled in one MCU · local chain only.

use ash::vk;
use ash::Device;

use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::display_res_intsct_rt_pkgs::{
    DisplayCommandDefaultRt, DisplayRenderDefaultRt, VulkanDisplayDefaultRt,
};

// ── Traits hot → cold ───────────────────────────────────────────────────────

/// Catalog — command runtime bag · FIF count is setup intention → Handled.
pub trait CommandDefaultHandled {
    fn handled_assemble(
        device_extrl: &Device,
        command_pool_extrl: vk::CommandPool,
        frames_in_flight_stp: u32,
    ) -> DisplayCommandDefaultRt;
}

/// Catalog — display render serial bag · fixed default → Auto.
pub trait DisplayRenderDefaultAuto {
    fn auto_assemble() -> DisplayRenderDefaultRt;
}

/// Catalog — vulkan display ready bag (depends on command + render) → Auto pack.
pub trait VulkanDisplayRuntimeDefaultAuto {
    fn auto_assemble(
        command_rt: &DisplayCommandDefaultRt,
        display_render_default_rt: &DisplayRenderDefaultRt,
    ) -> VulkanDisplayDefaultRt;
}

// ── Impls ───────────────────────────────────────────────────────────────────

impl CommandDefaultHandled for DisplayCommandDefaultRt {
    fn handled_assemble(
        _device_extrl: &Device,
        _command_pool_extrl: vk::CommandPool,
        _frames_in_flight_stp: u32,
    ) -> DisplayCommandDefaultRt {
        DisplayCommandDefaultRt {
            recording_rt: false,
            desc: "command_rt",
        }
    }
}

impl DisplayRenderDefaultAuto for DisplayRenderDefaultRt {
    fn auto_assemble() -> DisplayRenderDefaultRt {
        DisplayRenderDefaultRt {
            frame_serial_rt: 0,
            desc: "display_render",
        }
    }
}

impl VulkanDisplayRuntimeDefaultAuto for VulkanDisplayDefaultRt {
    fn auto_assemble(
        _command_rt: &DisplayCommandDefaultRt,
        _display_render_rt: &DisplayRenderDefaultRt,
    ) -> VulkanDisplayDefaultRt {
        VulkanDisplayDefaultRt {
            ready_rt: true,
            desc: "vulkan_display_runtime",
        }
    }
}
