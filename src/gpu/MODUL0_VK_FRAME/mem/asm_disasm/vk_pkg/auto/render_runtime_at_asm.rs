use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::runtime::frame_res_intsct_rt_pkgs::FrameRenderDefaultRtPkg;

/// Catalog — Strategy=Default (FIX-089).
pub trait RenderRuntimeDefaultAuto {
    fn auto_assemble() -> FrameRenderDefaultRtPkg;
}

impl RenderRuntimeDefaultAuto for FrameRenderDefaultRtPkg {
    fn auto_assemble() -> FrameRenderDefaultRtPkg {
        FrameRenderDefaultRtPkg {
            clear_color_rt: [0.0, 0.0, 0.0, 1.0],
            desc: "render_runtime",
        }
    }
}