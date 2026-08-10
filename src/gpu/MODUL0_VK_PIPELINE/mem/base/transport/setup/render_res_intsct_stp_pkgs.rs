//! Base **render** setup intersection — general knobs + triangle presets (FIX-120).

use ash::vk;

use super::op::RenderPassAttachmentLayoutStpPkgOp;

/// General render-pass recipe knobs.
pub struct RenderPassDefaultStpPkg {
    /// Operator / knob field `surface_format_op`.
    pub surface_format_op: vk::Format,
    /// Operator / knob field `sample_count_op`.
    pub sample_count_op: vk::SampleCountFlags,
    /// Operator / knob field `attachment_layout_op`.
    pub attachment_layout_op: RenderPassAttachmentLayoutStpPkgOp,
    /// Operator / knob field `depth_format_op`.
    pub depth_format_op: vk::Format,
    /// Operator / knob field `color_layout_op`.
    pub color_layout_op: vk::ImageLayout,
    /// Operator / knob field `depth_layout_op`.
    pub depth_layout_op: vk::ImageLayout,
    /// Operator / knob field `present_layout_op`.
    pub present_layout_op: vk::ImageLayout,
    /// Operator / knob field `initial_layout_op`.
    pub initial_layout_op: vk::ImageLayout,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// General graphics pipeline recipe knobs.
pub struct GraphicsPipelineDefaultStpPkg {
    /// Operator / knob field `sample_count_op`.
    pub sample_count_op: vk::SampleCountFlags,
    /// Operator / knob field `topology_op`.
    pub topology_op: vk::PrimitiveTopology,
    /// Operator / knob field `polygon_mode_op`.
    pub polygon_mode_op: vk::PolygonMode,
    /// Operator / knob field `cull_mode_op`.
    pub cull_mode_op: vk::CullModeFlags,
    /// Operator / knob field `front_face_op`.
    pub front_face_op: vk::FrontFace,
    /// Operator / knob field `depth_compare_op`.
    pub depth_compare_op: vk::CompareOp,
    /// Operator / knob field `color_write_mask_op`.
    pub color_write_mask_op: vk::ColorComponentFlags,
    /// Setup phase field `extent_width_stp`.
    pub extent_width_stp: u32,
    /// Setup phase field `extent_height_stp`.
    pub extent_height_stp: u32,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Descriptor set layout binding recipe (flexible vector).
pub struct DescriptorSetLayoutDefaultStpPkg {
    /// Operator / knob field `bindings_op`.
    pub bindings_op: Vec<vk::DescriptorSetLayoutBinding<'static>>,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Descriptor pool sizes recipe.
pub struct DescriptorPoolDefaultStpPkg {
    /// Setup phase field `max_sets_stp`.
    pub max_sets_stp: u32,
    /// Operator / knob field `pool_sizes_op`.
    pub pool_sizes_op: Vec<vk::DescriptorPoolSize>,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

/// Sampler recipe.
pub struct SamplerDefaultStpPkg {
    /// Operator / knob field `mag_filter_op`.
    pub mag_filter_op: vk::Filter,
    /// Operator / knob field `min_filter_op`.
    pub min_filter_op: vk::Filter,
    /// Operator / knob field `address_mode_u_op`.
    pub address_mode_u_op: vk::SamplerAddressMode,
    /// Operator / knob field `address_mode_v_op`.
    pub address_mode_v_op: vk::SamplerAddressMode,
    /// Operator / knob field `address_mode_w_op`.
    pub address_mode_w_op: vk::SamplerAddressMode,
    /// Setup phase field `anisotropy_enable_stp`.
    pub anisotropy_enable_stp: bool,
    /// Setup phase field `max_anisotropy_stp`.
    pub max_anisotropy_stp: f32,
    /// Human-readable bag descriptor (`&'static str` protocol tag).
    pub desc: &'static str,
}

// Triangle aliases (same layout as general).
/// `RenderPassTriangleStpPkg` — type alias (render pass triangle stp pkg).
/// Defined in `gpu/MODUL0_VK_PIPELINE/mem/base/transport/setup`.
pub type RenderPassTriangleStpPkg = RenderPassDefaultStpPkg;
/// `PipelineTriangleStpPkg` — type alias (pipeline triangle stp pkg).
/// Defined in `gpu/MODUL0_VK_PIPELINE/mem/base/transport/setup`.
pub type PipelineTriangleStpPkg = GraphicsPipelineDefaultStpPkg;
