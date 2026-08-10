<img src="logo.png" alt="modul" width="280" align="left">

<br clear="all">

---

# modul

Rust library that structures [Vulkan](https://www.vulkan.org/) work on top of [ash](https://crates.io/crates/ash).

Vulkan setup tends to scatter across large free functions and ad-hoc types. **modul** keeps each subsystem (swapchain, pipeline, frame pacing, display recording, mesh upload) as a separate unit with a fixed internal layout and a narrow public surface. The goal is readable ownership of GPU resources, not a game engine or an application framework.

Author: [Daniil Vasilev](https://github.com/peilerx) · license: [Apache-2.0](LICENSE)

---

## What you get

| Piece | Role |
|-------|------|
| `MODUL0_VK_SWAPCHAIN` | Instance, device, surface, swapchain, present mode |
| `MODUL0_VK_PIPELINE` | Render pass, shader modules, graphics pipelines |
| `MODUL0_VK_FRAME` | Frames-in-flight |
| `MODUL0_VK_DISPLAY` | Command buffer recording |
| `MODUL0_VK_MESH` | Mesh upload, instancing, push constants |
| `range/cubes` | Reference app: large instanced cube field (FIFO present) |
| `shader/cubes.*` | Product vertex/fragment shaders (+ SPIR-V) |

Each `MODUL0_*` unit follows the same disk shape:

```text
mem/    # bags, assembler ranks (setup / runtime / buffers)
conv/   # ports — the only intended external entry (import_* / export_*)
proc/   # domain logic (record, draw, free)
```

The external surface of a unit is called a **PTP** (protocol transport port): peers and the app shell talk through ports, not by reaching into `mem/`. Shared helpers live under `common/`; short architecture notes live in rustdoc under `modul::canon` and in [docs/GLOSSARY.md](docs/GLOSSARY.md).

---

## Build and run the cubes demo

This repository is meant to sit in a Cargo workspace that also provides `ash`, `winit`, and the `cubes` package (see the monorepo `modul-project/` layout, or add a local workspace root that lists `modul` and `modul/range/cubes`).

```bash
# 1_000_000 cubes, release, FIFO vsync
cargo run -p cubes --release

# smaller count
CUBES_COUNT=1000 cargo run -p cubes --release
```

Controls: left mouse — orbit · mouse wheel — zoom · Esc — quit.

Default present mode is FIFO (`SrgbFifo`). Mesh path is the direct solid-instance path used by the cubes demo.

---

## Use as a library

```toml
[dependencies]
modul = { path = "path/to/modul" }
# plus ash / window crates required by your shell
```

Typical product boot order (as in `range/cubes`):

1. Swapchain — surface, device, present  
2. Pipeline — render pass and cubes pipelines  
3. Frame — frames-in-flight  
4. Display — record  
5. Mesh — instances and push constants  

Session helpers in the demo: `assemble_tandem_session` → `run_tandem_pulse` → `free_tandem`.

---

## Repository layout

```text
modul/
  src/
    canon/       # architecture glossary (docs only)
    common/      # ModulResult, SPIR-V load, memory helpers
    cpu/         # reserved lane
    gpu/         # MODUL0_VK_* units
  shader/        # cubes.vert / cubes.frag (+ .spv)
  range/cubes/   # reference application
  docs/          # rustdoc header, glossary
```

### API docs

From the Cargo workspace root that contains this crate:

```bash
RUSTDOCFLAGS="--html-in-header modul/docs/rustdoc-header.html" \
  cargo doc -p modul --no-deps --document-private-items
# → target/doc/modul/index.html
# glossary → target/doc/modul/canon/index.html
```

### Architecture checks

Presets for [modlin](https://github.com/peilerx/modlin) are declared in this crate’s `Cargo.toml` under `[package.metadata.modlin.*]`. Example:

```bash
# if modlin binary is available
modlin report --path .
```

---

## Requirements

- Rust 2021  
- Vulkan-capable GPU and drivers  
- Linux is the primary target; the cubes demo uses winit  

---

## License

Apache License 2.0 — see [LICENSE](LICENSE) and [NOTICE](NOTICE).

The names **modul** and **modlin** are product identity and are not licensed as the name of a fork, crate, or service. Derivatives should use a different product name. Attribution must retain Apache-2.0 notices.

---

## Related

| Project | Description |
|---------|-------------|
| [modlin](https://github.com/peilerx/modlin) | Architectural linter used to keep modul’s layout and naming consistent |
| `cubes` | Reference Vulkan app under `range/cubes` |
