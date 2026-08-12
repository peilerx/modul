//! K re-export W. Atoms *`RtPkg`/*`StpPkg` · cargo *`RtCrg`/*`StpCrg`/*`MxCrg`.

pub use crate::gpu::MODUL0_VK_DISPLAY::mem::base::transport::{
    DisplayInputDefaultRtPkg, DisplayDefaultRtCrg, DisplayPresentPrt,
};
pub use crate::gpu::MODUL0_VK_FRAME::mem::base::transport::{
    FrameRenderDefaultRtPkg, FrameDefaultRtCrg, FrameFifPrt,
    FrameSlotDefaultRtPkg, FrameSyncDefaultRtPkg,
};
pub use crate::gpu::MODUL0_VK_PIPELINE::mem::base::transport::{
    PipelineTriangleRtPkg, RenderLanePrt, RenderPassTriangleRtPkg, RendererDefaultRtCrg,
    ShadersTriangleRtPkg,
};
pub use crate::gpu::MODUL0_VK_SWAPCHAIN::mem::base::transport::{
    SwapchainAssemblyPrt, DepthImagesDefaultRtPkg, DeviceDefaultRtPkg, FramebufferDefaultRtPkg,
    MsaaColorDefaultRtPkg, PhysicalDeviceDefaultRtPkg, PresentationDefaultRtCrg, PresentationPrt,
    SampleCountDefaultRtPkg, SurfaceDefaultRtPkg, SwapchainRtCrg,
    SwapchainCommandPoolDefaultRtPkg, SwapchainDefaultRtPkg, SwapchainImageViewsDefaultRtPkg,
    SwapchainLoaderDefaultRtPkg, SwapchainPrt,
};
// TandemSession* lives in modulcad (T.App) · not re-exported from modul lib
