//! MCU **pipeline_cache_res_intsct** — empty + seeded cache create paths (N.RES_INTSCT · FIX-120).
//! Multi-trait · one resource family · **one file** · ¬ cross-gen import.
//! Pure extrl peels → Auto class (M.5 · FIX-131).

use ash::vk;
use ash::Device;

use crate::{map_vk, ModulResult};

// ── Traits hot → cold ───────────────────────────────────────────────────────

/// Catalog — empty pipeline cache (hottest simple path).
pub trait PipelineCacheAuto {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Catalog — pipeline cache from initial data peel.
pub trait PipelineCacheSeededAuto {
    fn auto_assemble(device_extrl: &Device, initial_data_extrl: &[u8]) -> ModulResult<Self>
    where
        Self: Sized;
}

// ── Impls (same order) ──────────────────────────────────────────────────────

impl PipelineCacheAuto for vk::PipelineCache {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<Self> {
        let create_info = vk::PipelineCacheCreateInfo::default();
        map_vk(unsafe { device_extrl.create_pipeline_cache(&create_info, None) })
    }
}

impl PipelineCacheSeededAuto for vk::PipelineCache {
    fn auto_assemble(device_extrl: &Device, initial_data_extrl: &[u8]) -> ModulResult<Self> {
        let data_len_stp = initial_data_extrl.len();
        let create_info = match data_len_stp {
            0 => vk::PipelineCacheCreateInfo::default(),
            data_len_stp => {
                let _ = data_len_stp;
                vk::PipelineCacheCreateInfo::default().initial_data(initial_data_extrl)
            }
        };
        map_vk(unsafe { device_extrl.create_pipeline_cache(&create_info, None) })
    }
}
