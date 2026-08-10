//! Module `common/protocol/gpu`.
//!
//! Part of the modul Vulkan MCG layout (mem / conv / proc canon).

pub mod vk;

pub use vk::{
    SwapchainAssemblyPrt, DepthImagesDefaultRtPkg, DeviceDefaultRtPkg, DisplayInputDefaultRtPkg,
    DisplayDefaultRtCrg, DisplayPresentPrt, FrameRenderDefaultRtPkg,
    FrameDefaultRtCrg, FrameFifPrt, FrameSlotDefaultRtPkg, FrameSyncDefaultRtPkg,
    FramebufferDefaultRtPkg, MsaaColorDefaultRtPkg, PhysicalDeviceDefaultRtPkg,
    PipelineTriangleRtPkg, PresentationDefaultRtCrg, PresentationPrt, RenderLanePrt,
    RenderPassTriangleRtPkg, RendererDefaultRtCrg, SampleCountDefaultRtPkg, ShadersTriangleRtPkg,
    SurfaceDefaultRtPkg, SwapchainRtCrg, SwapchainCommandPoolDefaultRtPkg,
    SwapchainDefaultRtPkg, SwapchainImageViewsDefaultRtPkg, SwapchainLoaderDefaultRtPkg,
    SwapchainPrt,
};
