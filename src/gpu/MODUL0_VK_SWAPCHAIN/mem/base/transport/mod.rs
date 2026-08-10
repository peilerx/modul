//! Port module `gpu/MODUL0_VK_SWAPCHAIN/mem/base/transport`.
//!
//! PTP import/export free functions (FIX-128/130).

pub mod prt;
/// Submodule `runtime`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/mem/base/transport` under the mem/conv/proc MCG canon.
pub mod runtime;
/// Submodule `setup`.
/// Part of `gpu/MODUL0_VK_SWAPCHAIN/mem/base/transport` under the mem/conv/proc MCG canon.
pub mod setup;

pub use prt::{PresentationPrt, SwapchainAssemblyPrt, SwapchainPrt};
pub use runtime::{
    DeviceDefaultRtPkg, PhysicalDeviceDefaultRtPkg, SurfaceDefaultRtPkg,
    SwapchainCommandPoolDefaultRtPkg, SwapchainLoaderDefaultRtPkg, SwapchainRtCrg,
    DepthImagesDefaultRtPkg, FramebufferDefaultRtPkg, MsaaColorDefaultRtPkg,
    PresentationDefaultRtCrg, SampleCountDefaultRtPkg, SwapchainDefaultRtPkg,
    SwapchainImageViewsDefaultRtPkg,
};
pub use setup::{
    SurfaceWindowStpPkg, SwapchainAssemblyDefaultStpPkg,
    PresentationDefaultStpPkg, SwapchainDefaultStpPkg,
};
