//! Line GPU count arithmetic (P · no vk create).

/// Minimum float count for list (6) vs tris ribbons (9).
#[must_use]
pub const fn line_gpu_min_floats(as_tris_rt: bool) -> usize {
    if as_tris_rt {
        9
    } else {
        6
    }
}

/// `(vertex_count, line_count)` from position float len.
#[must_use]
pub const fn line_gpu_counts(positions_len_stp: usize, as_tris_rt: bool) -> (u32, u32) {
    let vertex_count_rt = (positions_len_stp / 3) as u32;
    let line_count_rt = if as_tris_rt {
        vertex_count_rt / 6
    } else {
        vertex_count_rt / 2
    };
    (vertex_count_rt, line_count_rt)
}

/// Whether rewrite is size/topology-compatible.
#[must_use]
pub const fn line_gpu_rewrite_ok(
    positions_len_stp: usize,
    as_tris_rt: bool,
    ready_rt: bool,
    bag_as_tris_rt: bool,
    bag_vertex_count_rt: u32,
) -> bool {
    let min_stp = line_gpu_min_floats(as_tris_rt);
    if positions_len_stp < min_stp || !ready_rt || bag_as_tris_rt != as_tris_rt {
        return false;
    }
    let (vertex_count_rt, _) = line_gpu_counts(positions_len_stp, as_tris_rt);
    bag_vertex_count_rt == vertex_count_rt
}
