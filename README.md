# modul

<!-- logo: place brand mark above this line -->

**Vulkan API extender** for Rust — a structured library layer on [ash](https://crates.io/crates/ash).

Not a game engine. Not a full application framework.  
**modul** organizes GPU work as **Modul Consistency Groups (MCG)**: closed factory units with explicit memory, conveyor (ports), and processing layers.

Current public cut: the **direct** product path used by the `cubes` etalon (instanced solid geometry, FIFO present, no Viewsor).

---

## Features

- Five live GPU MCGs: swapchain, pipeline, frame, display, mesh  
- **PTP** (Protocol Transport Port) — external API of each MCG via ports (`import_*` / `export_*`)  
- **Factory-Line** assembly: counted `import_for_asmN` slots, Auto | Handled ranks under `mem/asm_disasm/`  
- Product shaders: `shader/cubes.vert` / `cubes.frag` (+ SPIR-V)  
- Architecture enforceable by **modlin** presets in this crate’s `Cargo.toml`  
- Rustdoc with full API comments and canon glossary (`modul::canon`)

---

## Workspace layout

```text
modul/
  src/
    canon/          # MCG, PTP, phases, letters (docs only)
    common/         # ModulResult, SPIR-V, memory type, protocol re-exports
    cpu/            # empty lane in this cut
    gpu/
      MODUL0_VK_SWAPCHAIN/
      MODUL0_VK_PIPELINE/
      MODUL0_VK_FRAME/
      MODUL0_VK_DISPLAY/
      MODUL0_VK_MESH/
  shader/           # cubes.* only
  range/cubes/      # etalon app (workspace package `cubes`)
  docs/             # rustdoc header, glossary notes
  README.md
```

| Module | Role |
|--------|------|
| `gpu::MODUL0_VK_SWAPCHAIN` | Instance, device, surface, swapchain, presentation |
| `gpu::MODUL0_VK_PIPELINE` | Render pass, shader modules, graphics pipelines |
| `gpu::MODUL0_VK_FRAME` | Frames-in-flight (begin / end) |
| `gpu::MODUL0_VK_DISPLAY` | Record command buffers |
| `gpu::MODUL0_VK_MESH` | Mesh upload, instancing, steel/cubes push constants |
| `common` | Shared helpers and protocol peels |
| `canon` | Architecture glossary (rustdoc) |

---

## Quick start

From the Cargo workspace root (`modul-project/`):

```bash
# etalon: 1_000_000 unit cubes, direct path, FIFO vsync
cargo run -p cubes --release

# smaller lattice
CUBES_COUNT=1000 cargo run -p cubes --release
```

Controls (cubes): LMB orbit · wheel zoom · Esc quit.

### Use as a library

```toml
[dependencies]
modul = { path = "path/to/modul" }
# ash, winit, raw-window-handle as required by your shell
```

Boot order (direct product):

1. Swapchain MCG — surface + device + present (`SwapchainPrt::SrgbFifo` for vsync)  
2. Pipeline MCG — render pass + cubes pipelines  
3. Presentation  
4. Frame MCG — FIF  
5. Display MCG — record  
6. Mesh MCG — solid instances + push constants  

See `range/cubes` for a complete T.Hub (`assemble_tandem_session` · `run_tandem_pulse` · `free_tandem`).

---

## Concepts (short)

| Term | Meaning |
|------|---------|
| **MCG** | Modul Consistency Group — factory atom `MODUL0_{DOMAIN}` = M × C × P, closed boundary |
| **PTP** | Protocol Transport Port — external API of an MCG (intents, ports, peels) |
| **M / C / P** | Memory (`mem/`) · Conveyor (`conv/`) · Processing (`proc/`) |
| **Factory-Line** | Port assembly workplace (`import_for_asmN`, `IMPORT_*_FACTORY_LINE_N`) |
| **Stp / Rt / Prt / Bfr** | Setup · Runtime · Port intent · Buffer warehouse |
| **asm_disasm** | Auto \| Handled assemble and disassemble ranks |

Full glossary: rustdoc page **`modul::canon`**, or `docs/GLOSSARY.md`.

---

## Documentation

### Rustdoc (HTML)

```bash
cd modul-project
bash modul/scripts/build-docs.sh
# → target/doc/modul/index.html
# glossary → target/doc/modul/canon/index.html
```

Theme uses IBM Plex Sans (header: `docs/rustdoc-header.html`).

### Architecture lint

Presets live under `[package.metadata.modlin.*]` in this `Cargo.toml`.

```bash
# ship binary (from modlin)
../modlin/modlin-bin/modlin report --path .
# or in-tree copy after:
#   bash ../modlin/modlin-bin/sync-modlin-binary.sh --install-modul
./modlin report --path src
```

---

## Requirements

- Rust (edition 2021)  
- Vulkan-capable GPU and drivers  
- Linux (primary); windowing via winit in etalon apps  

---

## License and names

Licensed under the **Apache License, Version 2.0**.

- [LICENSE](LICENSE) — full license text  
- [NOTICE](NOTICE) — copyright and attribution  

You may use, modify, and distribute the software under Apache-2.0, including commercially.

The names **modul** and **modlin** are product identity and are **not** licensed for use as the name of a fork, crate, tool, or service. Use a different package and product name for derivatives. Attribution required where Apache-2.0 requires retention of notices.

---

## Related

| Crate | Role |
|-------|------|
| [modlin](../modlin/) | Architectural AST/FS linter for modul (and universal smell mode) |
| `cubes` | Direct Vulkan etalon app under `range/cubes` |
