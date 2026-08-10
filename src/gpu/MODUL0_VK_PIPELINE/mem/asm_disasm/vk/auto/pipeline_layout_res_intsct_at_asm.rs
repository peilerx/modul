//! MCU **pipeline_layout_res_intsct** — empty + full layout create (N.RES_INTSCT · N.FREQ · FIX-120).
//! Multi-trait · one file · local only · ¬ cross-gen import.

use ash::vk;
use ash::Device;

use crate::{map_vk, ModulResult};

// ── Traits hot → cold ───────────────────────────────────────────────────────

/// Catalog — empty pipeline layout (no descriptors / push constants).
pub trait PipelineLayoutEmptyAuto {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<Self>
    where
        Self: Sized;
}

/// Catalog — pipeline layout from set layouts + push constant ranges.
pub trait PipelineLayoutAuto {
    fn auto_assemble(
        device_extrl: &Device,
        set_layouts_extrl: &[vk::DescriptorSetLayout],
        push_constant_ranges_extrl: &[vk::PushConstantRange],
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

// ── Impls (same order) ──────────────────────────────────────────────────────

impl PipelineLayoutEmptyAuto for vk::PipelineLayout {
    fn auto_assemble(device_extrl: &Device) -> ModulResult<Self> {
        <Self as PipelineLayoutAuto>::auto_assemble(device_extrl, &[], &[])
    }
}

impl PipelineLayoutAuto for vk::PipelineLayout {
    fn auto_assemble(
        device_extrl: &Device,
        set_layouts_extrl: &[vk::DescriptorSetLayout],
        push_constant_ranges_extrl: &[vk::PushConstantRange],
    ) -> ModulResult<Self> {
        let create_info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(set_layouts_extrl)
            .push_constant_ranges(push_constant_ranges_extrl);
        map_vk(unsafe { device_extrl.create_pipeline_layout(&create_info, None) })
    }
}
