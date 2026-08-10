//! # `MeshDrawPrt` — how mesh cargo is assembled for draw
//!
//! Links host mesh peels to the Vulkan solid/line lanes.

/// Mesh draw session intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeshDrawPrt {
    /// Triangle list from a generic mesh peel (legacy etalon naming).
    #[default]
    TriangleList,
    /// Solid steel/cubes-shaded triangle list (`cubes` SPIR-V · pos+nrm VBO + instances).
    SteelSolid,
    /// Wireframe edges (reserved).
    Wireframe,
    /// Skip GPU mesh bind.
    Disabled,
}
