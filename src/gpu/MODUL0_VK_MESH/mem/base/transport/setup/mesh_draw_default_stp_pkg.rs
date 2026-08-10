/// Setup knobs for mesh draw lane.
pub struct MeshDrawDefaultStpPkg {
    /// 1 = triangle list · 2 = wireframe · 3 = steel solid · 0 = disabled.
    pub mode_stp: u32,
    /// Vertex count from CAD peel (external fact · not invented).
    pub vertex_count_stp: u32,
    /// Index count from CAD peel.
    pub index_count_stp: u32,
    /// Steel base RGB (cad_steel push constant).
    pub steel_r_stp: f32,
    /// Setup phase field `steel_g_stp`.
    pub steel_g_stp: f32,
    /// Setup phase field `steel_b_stp`.
    pub steel_b_stp: f32,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}
