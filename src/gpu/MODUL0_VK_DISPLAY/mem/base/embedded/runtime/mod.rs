//! DISPLAY embedded runtime — session bags moved to transport display_res_intsct (FIX-120).
//! Re-export for stable paths during migrate.
pub use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::runtime::{
    DisplayCommandDefaultRt, DisplayRenderDefaultRt, VulkanDisplayDefaultRt,
};
