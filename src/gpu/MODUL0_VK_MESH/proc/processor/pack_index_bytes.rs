//! Pack index buffers and steel count arithmetic (P · proc-only logic).

/// Little-endian u32 index stream for host-visible IBO upload.
#[must_use]
pub fn pack_u32_indices_to_bytes(indices_extrl: &[u32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(indices_extrl.len().saturating_mul(4));
    for index_rt in indices_extrl {
        out.extend_from_slice(&index_rt.to_ne_bytes());
    }
    out
}

/// `(vertex_count, index_count, triangle_count)` from steel interleaved bytes + indices.
#[must_use]
pub const fn steel_buffer_counts(vert_bytes_len_stp: usize, index_len_stp: usize) -> (u32, u32, u32) {
    let vertex_count_rt = (vert_bytes_len_stp / 24) as u32;
    let index_count_rt = index_len_stp as u32;
    let triangle_count_rt = index_count_rt / 3;
    (vertex_count_rt, index_count_rt, triangle_count_rt)
}
