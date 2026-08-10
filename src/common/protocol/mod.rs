//! # Protocol re-exports (letter **K** / **W**)
//!
//! Convenience path for transport peels and intents used across MCGs:
//! `*RtPkg` / `*StpPkg` atoms and `*RtCrg` cargo bags.
//!
//! Prefer importing from the owning MCG in new code; this module exists so
//! apps and cross-MCG ports share one namespace for wire types.

pub mod gpu;

pub use gpu::{
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
