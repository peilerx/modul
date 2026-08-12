//! # `MeshDrawPrt` — how mesh cargo is assembled for draw
//!
//! Intent enum only · mode peel ∈ `asm_disasm`.

/// Mesh draw session intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeshDrawPrt {
    /// Generic triangle list (no special material path).
    #[default]
    TRIANGLE_LIST,
    /// Solid shaded instanced mesh (`cubes` SPIR-V · pos+nrm + instance XYZ/LOD).
    SOLID,
    /// Wireframe edges (reserved).
    WIREFRAME,
    /// Skip GPU mesh bind.
    DISABLED,
}
