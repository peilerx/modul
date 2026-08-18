//! # GPU lane — Vulkan MCGs
//!
//! Hardware topology for GPU work: only `MODUL0_VK_*` factories (no EGUI, no UI).
//!
//! ## Live modules
//!
//! | MCG | Responsibility |
//! |-----|-----------------|
//! | `MODUL0_VK_SWAPCHAIN` | Instance, device, surface, swapchain, presentation resources |
//! | `MODUL0_VK_PIPELINE` | Render pass, shader modules, graphics pipelines (`cubes` solid) |
//! | `MODUL0_VK_FRAME` | Frames-in-flight: wait / acquire / submit / present fence cadence |
//! | `MODUL0_VK_DISPLAY` | Record command buffers for a frame (draw solid / lines) |
//! | `MODUL0_VK_MESH` | GPU VBO/IBO upload + instancing + push (host mesh ∈ `cpu::MODUL0_MESH`) |
//!
//! ## Session
//!
//! `session` is a **stub**: product session lives in the app T.Hub
//! (e.g. `range/cubes-auto` / `range/cubes-handled` TANDEM). Prefer app-shell assemble order over
//! `assemble_gpu_session`.
//!
//! ## Layering inside each MCG
//!
//! ```text
//! MODUL0_VK_* /
//!   mem/   base + asm_disasm (Auto|Handled ranks)
//!   conv/  port import|export
//!   proc/  processor | display
//! ```
//!
//! Apps should call **ports** (`conv::port`) and high-level proc helpers,
//! not poke raw assembler leaves unless extending the library.

/// Stub GPU session note — real control plane is app T.Hub.
pub mod session;

/// Display / record-frame MCG.
pub mod MODUL0_VK_DISPLAY;
/// Frames-in-flight MCG.
pub mod MODUL0_VK_FRAME;
/// Mesh upload + solid draw MCG.
pub mod MODUL0_VK_MESH;
/// Render pass + pipelines MCG.
pub mod MODUL0_VK_PIPELINE;
/// Bootstrap + swapchain presentation MCG.
pub mod MODUL0_VK_SWAPCHAIN;
