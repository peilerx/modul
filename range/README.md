# range — product etalons on modul

| Crate | Path | Session API |
|-------|------|-------------|
| **cubes-auto** | `range/cubes-auto` | `TandemSessionPrt` aggregate presets (+ env) |
| **cubes-handled** | `range/cubes-handled` | full `TandemSessionStpPkg` knobs (all Prt CAPS_SNAKE) |

```bash
cargo run -p cubes-auto --release
cargo run -p cubes-handled --release
```

Prt catalogs live under each MCG `mem/base/transport/prt/` and
`modul::tandem::MODUL0_TANDEM` session Prt/Stp.
