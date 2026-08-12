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
    TRIANGLE_SOLID_DEPTH,
    /// Solid · fill · back cull · depth less · 1×.
    TRIANGLE_SOLID_DEPTH_CULL_BACK,
    /// Wire · no cull · depth less · 1×.
    TRIANGLE_WIRE_DEPTH,
    /// Solid · fill · no cull · depth less · **MSAA 4×** (cubes etalon).
    TRIANGLE_SOLID_DEPTH_AA4,
    /// Solid · fill · no cull · depth less · MSAA 8×.
    TRIANGLE_SOLID_DEPTH_AA8,
    /// Wire · no cull · depth less · MSAA 4×.
    TRIANGLE_WIRE_DEPTH_AA4,
    /// Solid · depth always · 1× (debug overlay).
    TRIANGLE_SOLID_DEPTH_ALWAYS,
}
