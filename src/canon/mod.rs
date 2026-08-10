//! # Architecture canon: MCG & full abbreviation glossary
//!
//! Documentation-only module (no runtime logic).  
//! If you see **MCG**, **PTP**, `*StpPkg`, `asm_disasm`, **Slot-Factory-Line**, **modlin** — start here.
//!
//! **Rename note:** the external MCG surface was formerly called **PRA**
//! (*Protocol Resource Assembly*). It is now **PTP** —
//! **P**rotocol **T**ransport **P**ort.
//!
//! ---
//!
//! # What is an MCG?
//!
//! **MCG** = **M**odul **C**onsistency **G**roup (protocol definition).
//!
//! From the project protocol:
//!
//! ```text
//! CG   Consistency Group  ≔ tightly coupled resources · shared init-order · data locality
//! MCG  Modul Consistency Group ≔ factory atom M×C×P · ≥1 CG · closed on boundary
//! ```
//!
//! An MCG is a **self-sufficient coherent atom of digital matter**: tightly related
//! bags and processors, full **conv / mem / proc** structure, minimal cross-calls,
//! closed boundary. Externally it behaves like a dedicated silicon block; peers and
//! the app shell never dig into M internals — they use **PTP** only.
//!
//! On disk:
//!
//! ```text
//! src/{cpu|gpu}/MODUL0_{DOMAIN}/
//!   mem/     # Memory (M)
//!   conv/    # Conveyor (C)
//!   proc/    # Processing (P)
//! ```
//!
//! Examples: `MODUL0_VK_SWAPCHAIN`, `MODUL0_VK_MESH`.
//!
//! Internally: **MCG ≔ M × C × P** (factory atom; at least one **CG** inside).
//!
//! | Layer | Letter | Path | Branch / math? | Content |
//! |-------|--------|------|----------------|---------|
//! | Memory | **M** | `mem/` | No | Bags `*Stp`/`*Rt`; assembler `mem/asm_disasm/` |
//! | Conveyor | **C** | `conv/` | No (orchestration only) | Ports: `import_*` / `export_*` |
//! | Processing | **P** | `proc/` | **Yes** | Processors, record/draw, domain logic |
//!
//! ### MCG vs PTP
//!
//! | Term | Means | Who uses it |
//! |------|--------|-------------|
//! | **MCG** | **Modul Consistency Group** — internal factory atom (how bags/ranks are built) | Authors inside `MODUL0_*` |
//! | **PTP** | **External** API of that MCG (what peers / app shell may call) | Apps (`range/cubes` T.Hub), other MCGs |
//!
//! **PTP** = **P**rotocol **T**ransport **P**ort — contract surface:
//! intents (`*Prt`) + port free functions + asmed transport peels.  
//! Not an ECS product name. Not a new filesystem layer (paths stay under W + `conv/port`).
//!
//! ### MCG-unit maturity (FIX-134)
//!
//! | Band | Meaning |
//! |------|---------|
//! | **Full** | mem + conv + proc present |
//! | **Asm** | memory + assembler + port edge as needed |
//! | **Proc** | processing-heavy unit |
//! | **Thin** | minimal slice (stub / re-export) |
//!
//! Live GPU MCGs in this crate:  
//! `MODUL0_VK_SWAPCHAIN`, `MODUL0_VK_PIPELINE`, `MODUL0_VK_FRAME`,
//! `MODUL0_VK_DISPLAY`, `MODUL0_VK_MESH`.
//!
//! ---
//!
//! # Letter map (PROTOCOL)
//!
//! ## Layers inside an MCG
//!
//! | Letter | Name | Path / role |
//! |--------|------|-------------|
//! | **M** | Memory | `mem/` — internal bags; no domain branching |
//! | **C** | Conveyor | `conv/` — local orchestration; ports |
//! | **P** | Processing | `proc/` — owns logic (**L**); processors / display |
//!
//! ## Cross-cutting letters
//!
//! | Letter | Name | Meaning |
//! |--------|------|---------|
//! | **A** | All-modul iron | Static dispatch, ownership; **¬ dyn**, **¬ Arc&lt;Mutex&gt;** |
//! | **H** | Hardware topology | `src/{cpu,gpu,common}/` (+ app `tandem/`) |
//! | **N** | Naming / phase | Suffixes `_stp` `_rt` `_op` `_extrl`; bag PascalCase |
//! | **L** | Logic substrate | Who may compute / branch / transform (owned by **P**) |
//! | **Q** | Consistency | Cross-call / cross-import; who may see whose bags |
//! | **W** | Shareable memory | Protocol bags **on the wire** between MCGs (≠ full internal `*Rt`) |
//! | **K** | Common | Optional re-export of **W**; not a god-host of intents |
//! | **T** | Tandem / app shell | App hub (`run_tandem_pulse`, `TandemBfr`); consumes **PTP** only |
//! | **MCG** | Modul Consistency Group | Internal factory atom M×C×P under `MODUL0_{DOMAIN}` · ≥1 CG · closed boundary |
//! | **PTP** | Protocol Transport Port | External API of one MCG (replaces former **PRA**) |
//!
//! ## Executor / formal letters (tooling)
//!
//! | Letter | Name | Meaning |
//! |--------|------|---------|
//! | **E** | Lights / DoD | Red/Green, one-pass, crystal meta |
//! | **X** | Exception | Narrow carve-out; must cite a parent letter |
//! | **J** | Gate registry | **modlin** rule ids |
//! | **V** | Verify matrix | Coverage of checks |
//! | **S** | Supersession | History of law changes |
//! | **Y** | Examples | Illustrations only; never widen law |
//! | **I** | Index | Coverage map so substrate is not lost |
//!
//! Hierarchy (general → special):  
//! **A → H → N → L → Q → W → {M,C,P,K,T} → MCG / PTP → X / Y**
//!
//! ---
//!
//! # Consistency Group (CG)
//!
//! | Term | Meaning |
//! |------|---------|
//! | **CG** | **C**onsistency **G**roup — tightly coupled resources, shared init-order, data locality |
//! | **MCG** | **M**odul **C**onsistency **G**roup — factory atom built from M×C×P, containing ≥1 CG, closed on its boundary |
//!
//! Do not confuse **CG** (local coupling cluster) with **MCG** (whole factory unit).
//!
//! ---
//!
//! # Conveyor / port (C) abbreviations
//!
//! | Term | Expansion | Meaning |
//! |------|-----------|---------|
//! | **Port** | — | Free functions that import intents into setup or export asmed peels |
//! | **Factory-Line** | assembly line (was **Ceh**) | Workplace for asm / disasm / proc steps on a resource |
//! | **Slot-Factory-Line** | slotted factory-line | Typed slot where **PTP** edges attach (FIX-133) |
//! | **FACTORY_LINE_N** | count const | `IMPORT_*_FACTORY_LINE_N` = assemblies in one `import_for_asmN` body |
//! | **Transportable** | — | Subject trait: bag can move as a peel on the conveyor |
//! | **PortMatch** | — | `match` on `*Prt` writing setup knobs (`*Stp`) only |
//! | **import_*** | — | Port write; does not return owned product soup |
//! | **export_*** / **export_asmed*** | — | Port read; peel cargo for next MCG / hub |
//! | **peel** | — | Thin transport view of cargo (W), not full internal runtime graph |
//!
//! Forbidden path theater: empty `intent/` trees, `port/res`, legacy dual pins.
//!
//! ---
//!
//! # Memory / bag (M, N) abbreviations
//!
//! ### Phase suffixes
//!
//! | Token | Type form | Meaning |
//! |-------|-----------|---------|
//! | **stp** | `*Stp` | **S**e**t**u**p** — knobs before create |
//! | **rt** | `*Rt` | **R**un**t**ime — live handles / counts after create |
//! | **op** | `*Op` | Operator / discriminator (enums, match knobs) |
//! | **extrl** | `*_extrl` | **Exter**na**l** — raw Vulkan / host pointer |
//! | **prt** | `*Prt` | **P**o**rt** intent enum (module picture) |
//! | **bfr** | `*Bfr` | **B**u**f**fe**r** / warehouse of optional cargo slots |
//!
//! ### Package / cargo ranks (FIX-119/120)
//!
//! | Token | Type form | Meaning |
//! |-------|-----------|---------|
//! | **stp_pkg** | `*StpPkg` | Setup package — one setup atom |
//! | **rt_pkg** | `*RtPkg` | Runtime package — one runtime atom |
//! | **stp_crg** | `*StpCrg` | Setup cargo — only setup packages inside |
//! | **rt_crg** | `*RtCrg` | Runtime cargo — only runtime packages inside |
//! | **mx_crg** | `*MxCrg` | **M**i**x**ed cargo |
//! | **stp_pkg_op** | `*StpPkgOp` | Setup package operator |
//!
//! ### Assembler path (FIX-129)
//!
//! | Term | Meaning |
//! |------|---------|
//! | **asm_disasm** | Assemble / disassemble ranks (replaces forbidden `mem/generator`) |
//! | **Auto** | Catalog create without external peels (`auto_assemble`) |
//! | **Handled** | Create from imported knobs (`handled_assemble`) |
//! | **\*_at_asm** | Auto catalog leaf stem |
//! | **\*_hld_asm** | Handled catalog leaf stem |
//! | **vk** | Rank: single Vulkan object |
//! | **vk_pkg** | Rank: package of related objects |
//! | **vk_crg** | Rank: cargo composition |
//! | **vk_bfr** | Rank: buffer / warehouse assemble |
//! | **disassemble** | Reverse of assemble; free resources |
//!
//! ---
//!
//! # App shell / pulse (T)
//!
//! | Term | Meaning |
//! |------|---------|
//! | **T.Hub** | App tandem hub (`MODUL0_TANDEM` in `range/cubes`) |
//! | **TandemBfr** | Hub warehouse after **PTP** boot |
//! | **pulse / takt** | One discrete product frame (`run_tandem_pulse`) |
//! | **tempo** | Cadence of pulses |
//! | **Session** | Snapshot product state — not a long-lived type inside VK MCGs (FIX-127) |
//! | **FIF** | Frames In Flight |
//! | **Direct** | Product path without Viewsor / predictor (this published cut) |
//! | **MSAA** | Multi-Sample Anti-Aliasing (e.g. `TriangleSolidDepthAa4`) |
//! | **FIFO / MAILBOX** | Vulkan present modes (vsync vs uncapped) |
//!
//! ---
//!
//! # Hardware topology (H)
//!
//! | Path | Role |
//! |------|------|
//! | `src/gpu/` | GPU lane — live `MODUL0_VK_*` |
//! | `src/cpu/` | CPU lane — empty in this cut |
//! | `src/common/` | Shared helpers |
//! | app `tandem/` | T.Hub shell (outside lib internals) |
//!
//! ---
//!
//! # Tooling
//!
//! | Term | Meaning |
//! |------|---------|
//! | **modlin** | **modul** + **lin**ter — architectural AST/FS linter |
//! | **Red / Green** | Binary finding severities (FIX-107); DoD = zero Red |
//! | **preset** | `[package.metadata.modlin.preset.*]` scope in Cargo.toml |
//! | **gate** | One modlin rule (`check_*` + rule id) |
//! | **DoD** | Definition of Done |
//! | **GEAR** | Study cut: direct Vulkan cubes path |
//! | **ash** | Rust Vulkan bindings used by modul |
//! | **SPIR-V** | Shader intermediate; `cubes.vert.spv` / `cubes.frag.spv` |
//! | **VBO / IBO** | Vertex / index buffer objects |
//! | **AABB** | Axis-aligned bounding box (orbit fit) |
//!
//! ---
//!
//! # Naming recipe
//!
//! ```text
//! {Subject}{Strategy?}{Phase}
//!
//! MeshGpuDefaultRtPkg
//!   │       │       └── Phase = runtime package
//!   │       └── Strategy = Default
//!   └── Subject = MeshGpu
//! ```
//!
//! Field suffixes follow the same phases: `extent_rt`, `device_extrl`, `mode_op`.
//!
//! ---
//!
//! # Data flow (one MCG + PTP)
//!
//! ```text
//!   T.Hub / peer
//!        │  PTP (import_* · *Prt · export_asmed*)
//!        ▼
//!   conv/port  ──writes──►  *Stp knobs (mem)
//!        │
//!        ▼
//!   asm_disasm Auto|Handled  ──►  *Rt / *RtPkg / *RtCrg
//!        │
//!        ▼
//!   proc (record / draw / math)
//!        │
//!        ▼
//!   export peels  ──PTP──►  next MCG or T.Hub
//! ```
//!
//! ---
//!
//! # Historical alias
//!
//! | Old | New |
//! |-----|-----|
//! | **PRA** (Protocol Resource Assembly) | **PTP** (Protocol Transport Port) |
//! | `mem/generator` | `mem/asm_disasm` (FIX-129) |
//!
//! ---
//!
//! # See also
//!
//! - Crate root — product overview and boot order  
//! - [`crate::gpu`] — live Vulkan MCG table  
//! - Repo `docs/MODUL-PROJECT-PROTOCOL.md` — full law text  
//! - **modlin** package docs — gates that enforce this canon  

/// Marker type so rustdoc lists **canon** as a first-class page.
///
/// Never constructed at runtime. Open the **module documentation** above for
/// the full MCG definition and abbreviation glossary (**PTP**, phases, ranks).
#[derive(Debug, Clone, Copy)]
pub struct CanonGlossary;
