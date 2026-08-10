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
    /// Runtime phase field `steel_r_rt`.
    pub steel_r_rt: f32,
    /// Runtime phase field `steel_g_rt`.
    pub steel_g_rt: f32,
    /// Runtime phase field `steel_b_rt`.
    pub steel_b_rt: f32,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
