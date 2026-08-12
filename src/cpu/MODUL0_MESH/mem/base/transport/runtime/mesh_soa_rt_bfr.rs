//! Minimal SoA mesh bag for GPU upload (standalone mesh bag).

/// Positions + triangle indices (host) · optional per-instance world offsets.
pub struct MeshSoaRtBfr {
    /// Public field `pos_xs`.
    pub pos_xs: Vec<f32>,
    /// Public field `pos_ys`.
    pub pos_ys: Vec<f32>,
    /// Public field `pos_zs`.
    pub pos_zs: Vec<f32>,
    /// Public field `indices`.
    pub indices: Vec<u32>,
    /// World translation per instance (xyz). Empty → one instance at origin.
    pub inst_xs: Vec<f32>,
    /// Public field `inst_ys`.
    pub inst_ys: Vec<f32>,
    /// Public field `inst_zs`.
    pub inst_zs: Vec<f32>,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
