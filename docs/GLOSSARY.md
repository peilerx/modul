# modul glossary — MCG & abbreviations

Source of truth for rustdoc: **`modul::canon`** (`src/canon/mod.rs`).  
Open HTML: `target/doc/modul/canon/index.html`.

## Core

| Term | Expansion | Meaning |
|------|-----------|---------|
| **MCG** | **Modul Consistency Group** | Factory atom `MODUL0_{DOMAIN}` = **M×C×P** · ≥1 **CG** · closed boundary (protocol) |
| **CG** | Consistency Group | Tightly coupled resources · shared init-order · data locality (inside an MCG) |
| **PTP** | Protocol Transport Port | **External** API of one MCG (intents + ports + peels) |
| **PRA** | *(retired)* Protocol Resource Assembly | **Renamed to PTP** |

## Layers

| Letter | Name | Path |
|--------|------|------|
| **M** | Memory | `mem/` |
| **C** | Conveyor | `conv/` |
| **P** | Processing | `proc/` |

## Factory-Line (was Ceh)

| Term | Meaning |
|------|---------|
| **Factory-Line** | Assembly workplace: asm · disasm · proc steps on a resource (replaces **Ceh**) |
| **Slot-Factory-Line** | Typed port slot for **PTP** edges (`import_for_asmN`) |
| **FACTORY_LINE_N** | Const on subject port: assembly count in one import body |

## Cross-cutting letters

**A** all-modul iron · **H** hardware · **N** naming · **L** logic · **Q** consistency ·  
**W** shareable wire memory · **K** common re-export · **T** tandem app shell ·  
**E** lights/DoD · **X** exception · **J** modlin gates · **V** verify · **S** supersession · **Y** examples · **I** index

## Phases & ranks

**Stp** setup · **Rt** runtime · **Op** operator · **extrl** external handle ·  
**Prt** port intent · **Bfr** buffer warehouse ·  
**StpPkg / RtPkg / RtCrg / StpCrg / MxCrg** package/cargo ranks ·  
**asm_disasm** Auto|Handled assemble · **vk / vk_pkg / vk_crg / vk_bfr** ranks

## App

**T.Hub** · **TandemBfr** · **pulse/takt** · **FIF** · **Direct** · **FIFO/MAILBOX** present modes

## Tooling

**modlin** · **Red/Green** · **preset** · **gate** · **DoD** · **ash** · **SPIR-V** · **VBO/IBO**
