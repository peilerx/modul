# modul/range

Product etalons on **modul** — real sessions, not toy wrappers.

| Crate | Role |
|-------|------|
| `cubes` | Direct instanced cubes (Vulkan · TANDEM · 1M default) |

## Design stance (why this is not a “simple Vulkan wrapper”)

modul organizes GPU work as **Assembly Buffers + protocol intents** (`*Bfr`, `*Prt`, Factory-Line imports). The etalon keeps that surface **visible**:

1. **Session** (`assemble_tandem_session`) — ordered PTP imports: swapchain → renderer → presentation → frame → display → mesh.  
2. **Pulse** (`run_tandem_pulse`) — explicit `begin_frame` → `record_frame_with_serial` → `end_frame` with peels (`mesh_gpu_rt`, `mesh_push_rt`).  
3. **LOD / variants** — extend by changing assembler policies and peels inside pulse, not by adding a second hidden renderer API.

A monomorphic `TandemBuilder::with_*` facade can *call* these assemblers for convenience, but must not replace them: hiding FIF, present mode, render pass, and push constants would collapse modul into OpenGL-style opacity.

Flexibility target: enough surface to prototype many LOD strategies (instancing, push constants, multi-pass, later meshlets) without 3400 lines of raw ash — and without zero control.

## Run

```bash
# from modul-project workspace root
cargo run -p cubes --release
CUBES_COUNT=10000 cargo run -p cubes --release
```

Controls: LMB orbit · wheel zoom · Esc quit. Window resize rebuilds the session (extent is fixed at assemble).
