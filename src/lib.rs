//! # modul — Vulkan API extender
//!
//! **modul** is a structured library layer on top of the Vulkan API ([`ash`]).
//! It is **not** a game engine and **not** a replacement for the Vulkan specification.
//! It is an **MCG-oriented** layout for large GPU codebases: explicit ownership,
//! phases, and boundaries.
//!
//! ## Start here: MCG & abbreviations
//!
//! Full glossary (MCG, **PTP**, phases, ranks, letters **A–Y**):
//! **[`canon`]** — *Architecture canon: MCG & full abbreviation glossary*.
//!
//! | Term | Short |
//! |------|--------|
//! | **MCG** | **Modul Consistency Group** — factory atom `MODUL0_{DOMAIN}` = **M×C×P** · ≥1 CG · closed boundary |
//! | **PTP** | Protocol Transport Port — **external** API of an MCG (intents + ports + peels) |
//! | **M / C / P** | Memory `mem/` · Conveyor `conv/` · Processing `proc/` |
//! | **Stp / Rt / Prt / Bfr** | Setup · Runtime · Port intent · Buffer warehouse |
//! | **`asm_disasm`** | Auto\|Handled assemble/disassemble ranks (not legacy `generator`) |
//!
//! **PTP** replaces the former name **PRA** (*Protocol Resource Assembly*).
//!
//! ## What you get
//!
//! | Area | Content |
//! |------|---------|
//! | **GPU MCGs** | `gpu::MODUL0_VK_*` — swapchain, pipeline, frame, display, mesh |
//! | **CPU MCGs** | `cpu::MODUL0_MESH` — host SoA lattice + pack (no Vulkan) |
//! | **Common** | [`ModulResult`], SPIR-V helper, memory type pick, tracing |
//! | **Canon docs** | [`canon`] — MCG + every protocol abbreviation |
//!
//! ## Architecture (layers)
//!
//! | Layer | Path | Role |
//! |-------|------|------|
//! | **M** Memory | `mem/` | Bags setup/runtime; `mem/asm_disasm/` assembler |
//! | **C** Conveyor | `conv/` | **PTP** ports: `import_*` / `export_*` |
//! | **P** Processing | `proc/` | Branching, math, record/draw |
//!
//! - **PTP** — what apps call (not internal M bags)
//! - **Slot-Factory-Line** — typed port slots for PTP edges
//! - **Auto \| Handled** — assembler ranks under `asm_disasm/{vk,vk_pkg,vk_crg,vk_bfr}/`
//! - **¬ dyn / ¬ Arc&lt;Mutex&gt;** — static dispatch (**modlin**)
//! - **Direct path** — swapchain → pipeline → presentation → frame → display → mesh
//!
//! Forbidden (v5): `mem/generator`, empty `intent/`, `port/res`.
//!
//! ## Typical app boot (direct cubes)
//!
//! Etalon: `modul/range/cubes`.
//!
//! 1. `MODUL0_VK_SWAPCHAIN` — device/surface/swapchain (e.g. **FIFO** vsync)
//! 2. `MODUL0_VK_PIPELINE` — render pass + `cubes` SPIR-V pipelines
//! 3. Presentation (swapchain presentation port)
//! 4. `MODUL0_VK_FRAME` — FIF begin/end
//! 5. `MODUL0_VK_DISPLAY` — record command buffers
//! 6. `cpu::MODUL0_MESH` — host SoA mesh + pack bytes
//! 7. `MODUL0_VK_MESH` — VBO/IBO instances + steel push constants
//!
//! ## Shaders
//!
//! `modul/shader/cubes.{vert,frag}` (+ `.spv`), embedded via `include_bytes!`.
//!
//! ## Linting
//!
//! Presets: this crate’s `Cargo.toml` `[package.metadata.modlin.*]`.\
//! Binary: `modlin/modlin-bin/`.
//!
//! ## Crate map
//!
//! - [`canon`] — **MCG + abbreviations (PTP, phases, letters)**
//! - `common` — results, protocol re-exports, SPIR-V, tracing
//! - `gpu` — live Vulkan MCGs
//! - `tandem` — product hubs (`MODUL0_TANDEM`) composing GPU MCGs
//! - `cpu` — empty lane reserved for future CAD

#![allow(
    non_snake_case,
    reason = "MODUL0_{DOMAIN} CAPS segments per FIX-073 / Factory Mind"
)]
#![allow(
    non_camel_case_types,
    reason = "Prt intent variants use CAPS_SNAKE_CASE as product lever lists"
)]
#![allow(missing_docs)]

/// Architecture canon: **MCG**, **PTP**, phase suffixes, protocol letters.
///
/// Open this module’s rustdoc page for the full glossary.
pub mod canon;

/// Shared helpers (result type, SPIR-V, memory type, tracing, protocol re-exports).
pub mod common;
/// CPU hardware lane — host MCGs (`MODUL0_MESH` SoA / pack).
pub mod cpu;
/// GPU hardware lane — all live `MODUL0_VK_*` MCGs.
pub mod gpu;
/// Product session hubs (compose GPU MCGs; dual Prt + Stp knobs).
pub mod tandem;

pub use common::{
    assemble_shader_spv, find_vk_memory_type, from_err, map_vk, trace_deep, trace_emit,
    trace_enabled, trace_init_from_env, trace_mesh_stats, trace_paint_diag, trace_set_enabled,
    trace_sketch_loop, trace_throttle, trace_zbuffer_coverage, ModulResult,
};
