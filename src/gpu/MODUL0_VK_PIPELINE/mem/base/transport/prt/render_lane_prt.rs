//! # `RenderLanePrt` — full renderer lane picture (intent protocol)
//!
//! One intent family for this MCG — **not** a 1:1 Vulkan enum dump.
//! Aggregates the op-group for pass + graphics pipeline setup:
//! samples · attachment layout · depth compare · cull · polygon · topology.
//!
//! | Rank | Who consumes this Prt |
//! |------|------------------------|
//! | **Auto** | `RenderPassTriangleStpPkg` / `PipelineTriangleStpPkg` · preset table in `asm_disasm/…/render_lane_stp_at_asm` |
//! | **Port** | `RendererTransportable::import_render_lane_for_asm1` · writes *Stp onto Bfr |
//! | **Handled Rt** | later `RenderPassTriangleHandled` / pipeline Handled · knobs from those *Stp |
//!
//! Custom/non-preset knobs would need a Handled *Stp path (explicit ops bag);
//! product presets stay on Auto.

/// Named full-lane pictures of `MODUL0_VK_PIPELINE` (closed gestalt each arm).
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
