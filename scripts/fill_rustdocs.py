#!/usr/bin/env python3
"""Insert /// documentation on every undocumented pub item in modul/src."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "src"

ITEM_RE = re.compile(
    r"""^(?P<indent>\s*)
        (?P<vis>pub(?:\([^)]*\))?\s+)?
        (?P<qual>(?:async\s+|const\s+|unsafe\s+|extern\s+(?:"[^"]*"\s+)?)*(?:fn|struct|enum|trait|type|const|static)\s+)
        (?P<name>[A-Za-z_][A-Za-z0-9_]*)
    """,
    re.VERBOSE,
)

# enum variant: Name, or Name(
VARIANT_RE = re.compile(
    r"^(?P<indent>\s+)(?P<name>[A-Z][A-Za-z0-9_]*)\s*(?P<rest>[,({]|\s*=)"
)

MOD_RE = re.compile(r"^(?P<indent>\s*)pub\s+mod\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)")


def path_context(path: Path) -> str:
    try:
        rel = path.relative_to(ROOT)
    except ValueError:
        return ""
    parts = list(rel.parts[:-1])
    return "/".join(parts)


def split_camel(name: str) -> list[str]:
    # Keep ALLCAPS tokens
    if name.isupper() and len(name) > 1:
        return [name]
    parts = re.findall(r"[A-Z]+(?=[A-Z][a-z]|[0-9]|$)|[A-Z]?[a-z]+|[0-9]+", name)
    return parts or [name]


def humanize(name: str) -> str:
    parts = split_camel(name)
    return " ".join(p.lower() if not p.isupper() else p for p in parts)


def describe_item(kind: str, name: str, ctx: str, line: str) -> list[str]:
    ctx_bits = []
    if "MODUL0_VK_SWAPCHAIN" in ctx:
        ctx_bits.append("swapchain / device bootstrap MCG")
    elif "MODUL0_VK_PIPELINE" in ctx:
        ctx_bits.append("render-pass / graphics pipeline MCG")
    elif "MODUL0_VK_FRAME" in ctx:
        ctx_bits.append("frames-in-flight MCG")
    elif "MODUL0_VK_DISPLAY" in ctx:
        ctx_bits.append("command-buffer record / display MCG")
    elif "MODUL0_VK_MESH" in ctx:
        ctx_bits.append("mesh upload / solid draw MCG")
    elif "protocol" in ctx:
        ctx_bits.append("shared protocol peels")
    elif "common" in ctx:
        ctx_bits.append("common helpers")

    layer = None
    for L in ("mem", "conv", "proc"):
        if f"/{L}/" in f"/{ctx}/" or ctx.endswith(f"/{L}") or ctx == L:
            layer = L
            break

    phase = None
    for token, label in (
        ("StpPkg", "setup package bag"),
        ("RtPkg", "runtime package bag"),
        ("StpCrg", "setup cargo bag"),
        ("RtCrg", "runtime cargo bag"),
        ("MxCrg", "mixed cargo bag"),
        ("StpOp", "setup operator / knob enum"),
        ("Bfr", "buffer / warehouse bag"),
        ("Prt", "port intent enum"),
        ("Stp", "setup bag"),
        ("Rt", "runtime bag"),
        ("Op", "operator / discriminator"),
    ):
        if name.endswith(token) or name.endswith(token + "'"):
            phase = label
            break

    hum = humanize(name)
    lines: list[str] = []

    if kind == "struct":
        role = phase or "data structure"
        lines.append(f"`{name}` — {role} ({hum}).")
        if layer == "mem":
            lines.append("Memory-layer bag: owned fields, no product control flow.")
        elif layer == "conv":
            lines.append("Conveyor/port-related structure used when wiring PRA imports/exports.")
        elif layer == "proc":
            lines.append("Processing-layer structure used by processors / record-draw helpers.")
        if ctx_bits:
            lines.append(f"Belongs to: {', '.join(ctx_bits)}.")
        if "extrl" in line or name.endswith("Extrl") or "_extrl" in line:
            lines.append("Fields ending in `_extrl` hold raw Vulkan / external handles.")
        lines.append(f"Module path context: `{ctx or 'crate root'}`.")
    elif kind == "enum":
        role = phase or "enumeration"
        lines.append(f"`{name}` — {role} ({hum}).")
        if name.endswith("Prt"):
            lines.append(
                "Port intent (`*Prt`): closed module picture matched in `conv/port` to setup knobs."
            )
        elif name.endswith("Op"):
            lines.append("Operator / discriminator enum for setup or match knobs.")
        if ctx_bits:
            lines.append(f"Belongs to: {', '.join(ctx_bits)}.")
        lines.append(f"Module path context: `{ctx or 'crate root'}`.")
    elif kind == "trait":
        lines.append(f"`{name}` — trait ({hum}).")
        if "Auto" in name:
            lines.append(
                "Auto-assemble catalog trait: pure construction without external peels (FIX-129 Auto rank)."
            )
        elif "Handled" in name:
            lines.append(
                "Handled-assemble catalog trait: construction from imported knobs / peels (FIX-129 Handled rank)."
            )
        elif "Transportable" in name:
            lines.append("Transportable surface: import/export peels for PRA slot-ceh wiring.")
        else:
            lines.append("Implements a catalog or port contract for this MCG leaf.")
        if ctx_bits:
            lines.append(f"Belongs to: {', '.join(ctx_bits)}.")
        lines.append(f"Module path context: `{ctx or 'crate root'}`.")
    elif kind == "fn":
        lines.append(f"`{name}` — function ({hum}).")
        if name.startswith("auto_"):
            lines.append("Auto-rank assemble/disassemble entry.")
        elif name.startswith("handled_"):
            lines.append("Handled-rank assemble/disassemble entry.")
        elif name.startswith("import_"):
            lines.append("Port import: write intent/knobs into a buffer (never returns owned product soup).")
        elif name.startswith("export_"):
            lines.append("Port export: peel asmed cargo for the next MCG / app hub.")
        elif name.startswith("record_"):
            lines.append("Record/emit side-effect helper (draw, log, or metrics).")
        elif name.startswith("check_"):
            lines.append("Predicate / validation helper.")
        elif name.startswith("assemble_") or name.endswith("_assemble"):
            lines.append("Assemble Vulkan or host resources into bags/cargo.")
        elif name.startswith("free_") or "disassemble" in name:
            lines.append("Release / disassemble previously assembled resources.")
        elif name.startswith("begin_") or name.startswith("end_"):
            lines.append("Frame cadence helper (FIF begin/end).")
        elif name.startswith("map_") or name.startswith("from_"):
            lines.append("Conversion / mapping helper.")
        elif name.startswith("find_"):
            lines.append("Lookup helper.")
        elif name.startswith("trace_"):
            lines.append("Optional host tracing (env-gated).")
        else:
            lines.append("Public API entry for this module.")
        if "ModulResult" in line or "Result<" in line:
            lines.append("# Errors")
            lines.append("")
            lines.append("Returns [`ModulResult`](crate::ModulResult) / `Result` on Vulkan or validation failure.")
        if ctx_bits:
            lines.append(f"Belongs to: {', '.join(ctx_bits)}.")
    elif kind == "type":
        lines.append(f"`{name}` — type alias ({hum}).")
        lines.append(f"Defined in `{ctx or 'crate root'}`.")
    elif kind in ("const", "static"):
        lines.append(f"`{name}` — {kind} ({hum}).")
        lines.append(f"Module path context: `{ctx or 'crate root'}`.")
    elif kind == "mod":
        lines.append(f"Submodule `{name}`.")
        lines.append(f"Part of `{ctx or 'crate'}` under the mem/conv/proc MCG canon.")
    elif kind == "variant":
        lines.append(f"Enum variant `{name}` — {hum}.")
    else:
        lines.append(f"`{name}` — {kind}.")

    return lines


def has_doc_above(lines: list[str], idx: int) -> bool:
    j = idx - 1
    while j >= 0:
        s = lines[j].strip()
        if not s:
            j -= 1
            continue
        if s.startswith("///") or s.startswith("//!"):
            return True
        if s.startswith("#[") or s.startswith("#!["):
            j -= 1
            continue
        if s.startswith("//"):
            j -= 1
            continue
        return False
    return False


def attr_block_start(lines: list[str], idx: int) -> int:
    """Index where attributes for this item begin (insert docs here)."""
    j = idx
    while j > 0:
        prev = lines[j - 1].strip()
        if prev.startswith("#[") or prev.startswith("#!["):
            j -= 1
            # multi-line attributes: walk back until line with ]
            # simplistic: if line doesn't end with ] and starts with #, still attr
            continue
        if prev == "" or prev.startswith("//") and not prev.startswith("///"):
            # don't skip plain comments into previous item
            if prev.startswith("//") and not prev.startswith("///"):
                break
            if prev == "":
                break
        break
    return j


def process_file(path: Path) -> int:
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines(keepends=True)
    ctx = path_context(path)
    added = 0
    i = 0
    in_enum = 0  # brace depth tracking for enum body

    # Track enum bodies for variants
    enum_depth_stack: list[int] = []

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # track braces for enum variants (rough)
        if enum_depth_stack:
            enum_depth_stack[-1] += line.count("{") - line.count("}")
            if enum_depth_stack[-1] <= 0:
                enum_depth_stack.pop()

        mmod = MOD_RE.match(line)
        if mmod and "pub" in mmod.group(0):
            if not has_doc_above(lines, i):
                name = mmod.group("name")
                docs = describe_item("mod", name, ctx, line)
                indent = mmod.group("indent")
                # module prefers //! at file start; for pub mod use ///
                block = [f"{indent}/// {d}\n" if d else f"{indent}///\n" for d in docs]
                ins = attr_block_start(lines, i)
                for k, b in enumerate(block):
                    lines.insert(ins + k, b)
                added += 1
                i = ins + len(block) + 1
                continue

        m = ITEM_RE.match(line)
        if m and m.group("vis") and m.group("vis").startswith("pub"):
            kind_raw = m.group("qual")
            kind = (
                "fn"
                if "fn" in kind_raw
                else "struct"
                if "struct" in kind_raw
                else "enum"
                if "enum" in kind_raw
                else "trait"
                if "trait" in kind_raw
                else "type"
                if "type" in kind_raw
                else "const"
                if "const" in kind_raw
                else "static"
            )
            name = m.group("name")
            if not has_doc_above(lines, i):
                docs = describe_item(kind, name, ctx, line)
                indent = m.group("indent")
                block = []
                for d in docs:
                    if d == "# Errors":
                        block.append(f"{indent}/// # Errors\n")
                    elif d == "":
                        block.append(f"{indent}///\n")
                    else:
                        block.append(f"{indent}/// {d}\n")
                ins = attr_block_start(lines, i)
                for k, b in enumerate(block):
                    lines.insert(ins + k, b)
                added += 1
                # if enum, track body for variants
                if kind == "enum" and "{" in line:
                    enum_depth_stack.append(1)
                i = ins + len(block) + 1
                continue
            if kind == "enum" and "{" in line:
                enum_depth_stack.append(1)

        # enum variants
        if enum_depth_stack and enum_depth_stack[-1] >= 1:
            vm = VARIANT_RE.match(line)
            if vm and not stripped.startswith("//") and not stripped.startswith("#"):
                # skip if looks like field in struct - we're in enum
                name = vm.group("name")
                # skip common non-variants
                if name in ("Self", "Ok", "Err") and False:
                    pass
                if not has_doc_above(lines, i):
                    docs = describe_item("variant", name, ctx, line)
                    indent = vm.group("indent")
                    block = [f"{indent}/// {d}\n" for d in docs]
                    ins = attr_block_start(lines, i)
                    for k, b in enumerate(block):
                        lines.insert(ins + k, b)
                    added += 1
                    i = ins + len(block) + 1
                    continue

        i += 1

    if added:
        path.write_text("".join(lines), encoding="utf-8")
    return added


def main() -> int:
    total = 0
    files = 0
    for path in sorted(ROOT.rglob("*.rs")):
        n = process_file(path)
        if n:
            files += 1
            total += n
            print(f"+{n:4d}  {path.relative_to(ROOT)}")
    print(f"\nInserted {total} doc comments across {files} files")
    return 0


if __name__ == "__main__":
    sys.exit(main())
