//! Line push size · P.Processor.
//! Empty `LineGpuDefaultRtPkg` / `LinePushRt` constructors ∈
//! `mem/asm_disasm/vk_pkg/auto/line_gpu_default_rt_pkg_at_asm.rs`.

use crate::gpu::MODUL0_VK_MESH::mem::base::transport::runtime::line_gpu_default_rt_pkg::LinePushRt;

/// Push constant block size for line shaders.
pub const LINE_PUSH_RT_SIZE: u32 = core::mem::size_of::<LinePushRt>() as u32;
