//! # CPU lane — host MCGs
//!
//! | MCG | Role |
//! |-----|------|
//! | [`MODUL0_MESH`] | Host SoA mesh · lattice · pack to host bytes for GPU upload |

/// Host mesh factory (SoA · pack · lattice).
pub mod MODUL0_MESH;
