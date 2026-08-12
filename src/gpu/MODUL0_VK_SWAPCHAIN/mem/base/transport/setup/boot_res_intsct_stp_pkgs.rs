//! Base **boot** setup intersection (`M.BASE_RES_INTSCT` · FIX-120).
//! Window surface handles + swapchain assembly setup recipe.

/// Window handles for surface creation.
pub struct SurfaceWindowStpPkg {
    /// External / raw Vulkan handle or host pointer field `display_handle_extrl` (`display_handle` peel).
    pub display_handle_extrl: raw_window_handle::RawDisplayHandle,
    /// External / raw Vulkan handle or host pointer field `window_handle_extrl` (`window_handle` peel).
    pub window_handle_extrl: raw_window_handle::RawWindowHandle,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

impl SurfaceWindowStpPkg {
    /// Build setup bag from raw window handles (app peels winit / other → raw).
    #[must_use]
    pub const fn from_raw(
        display_handle_extrl: raw_window_handle::RawDisplayHandle,
        window_handle_extrl: raw_window_handle::RawWindowHandle,
        desc: &'static str,
    ) -> Self {
        Self {
            display_handle_extrl,
            window_handle_extrl,
            desc,
        }
    }
}

/// Swapchain assembly setup knobs only (FIX-086/091/097).
pub struct SwapchainAssemblyDefaultStpPkg {
    /// Setup phase field `validation_layers_stp`.
    pub validation_layers_stp: bool,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
