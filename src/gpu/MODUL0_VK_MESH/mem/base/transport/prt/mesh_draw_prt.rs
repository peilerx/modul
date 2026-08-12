//! # `MeshDrawPrt` — how mesh cargo is assembled for draw
//!
//! Links host mesh peels to the Vulkan solid/line lanes.

/// Mesh draw session intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeshDrawPrt {
    /// Generic triangle list (no special material path).
    #[default]
    TriangleList,
    /// Solid shaded instanced mesh (`cubes` SPIR-V · pos+nrm + instance XYZ/LOD).
    Solid,
    /// Wireframe edges (reserved).
    Wireframe,
    /// Skip GPU mesh bind.
    Disabled,
}
