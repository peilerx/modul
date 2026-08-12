/// Setup knobs for mesh draw lane.
pub struct MeshDrawDefaultStpPkg {
    /// 1 = triangle list · 2 = wireframe · 3 = steel solid · 0 = disabled.
    pub mode_stp: u32,
    /// Vertex count from mesh peel.
    pub vertex_count_stp: u32,
    /// Index count from mesh peel.
    pub index_count_stp: u32,
    /// Steel base RGB (`mesh_solid` push constant).
    pub base_r_stp: f32,
    /// Setup phase field `base_g_stp`.
    pub base_g_stp: f32,
    /// Setup phase field `base_b_stp`.
    pub base_b_stp: f32,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
