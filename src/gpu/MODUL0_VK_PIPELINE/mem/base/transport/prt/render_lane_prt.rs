//! # `RenderLanePrt` — full renderer lane picture
//!
//! One intent family for this MCG — not a 1:1 Vulkan enum dump.
//! PortMatch sets cull/topology/MSAA/depth knobs when assembling the renderer.

/// Named full-lane pictures of `MODUL0_VK_PIPELINE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderLanePrt {
    /// Solid · fill · no cull · depth less · 1× samples.
    #[default]
    TriangleSolidDepth,
    /// Solid · fill · back cull · depth less · 1×.
    TriangleSolidDepthCullBack,
    /// Wire · no cull · depth less · 1×.
    TriangleWireDepth,
    /// Solid · fill · no cull · depth less · **MSAA 4×** (cubes etalon).
    TriangleSolidDepthAa4,
    /// Solid · fill · no cull · depth less · MSAA 8×.
    TriangleSolidDepthAa8,
    /// Wire · no cull · depth less · MSAA 4×.
    TriangleWireDepthAa4,
    /// Solid · depth always · 1× (debug overlay).
    TriangleSolidDepthAlways,
}
