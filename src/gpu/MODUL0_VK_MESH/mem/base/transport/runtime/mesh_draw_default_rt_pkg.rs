/// Mesh draw peel — ready flags for frame bind (VBO upload later).
pub struct MeshDrawDefaultRtPkg {
    /// Runtime phase field `ready_rt`.
    pub ready_rt: bool,
    /// 1 tri · 2 wire · 3 steel solid · 0 off
    pub mode_rt: u32,
    /// Runtime phase field `vertex_count_rt`.
    pub vertex_count_rt: u32,
    /// Runtime phase field `index_count_rt`.
    pub index_count_rt: u32,
    /// Runtime phase field `triangle_count_rt`.
    pub triangle_count_rt: u32,
    /// Runtime phase field `base_r_rt`.
    pub base_r_rt: f32,
    /// Runtime phase field `base_g_rt`.
    pub base_g_rt: f32,
    /// Runtime phase field `base_b_rt`.
    pub base_b_rt: f32,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
