//! vk brick: `vk::Sampler`.

use ash::vk;
use ash::Device;

use crate::{map_vk, ModulResult};

/// Catalog — sampler from filter / address mode knobs.
pub trait SamplerHandled {
    fn handled_assemble(
        device_extrl: &Device,
        mag_filter_op: vk::Filter,
        min_filter_op: vk::Filter,
        address_mode_u_op: vk::SamplerAddressMode,
        address_mode_v_op: vk::SamplerAddressMode,
        address_mode_w_op: vk::SamplerAddressMode,
        anisotropy_enable_stp: bool,
        max_anisotropy_stp: f32,
    ) -> ModulResult<Self>
    where
        Self: Sized;
}

impl SamplerHandled for vk::Sampler {
    fn handled_assemble(
        device_extrl: &Device,
        mag_filter_op: vk::Filter,
        min_filter_op: vk::Filter,
        address_mode_u_op: vk::SamplerAddressMode,
        address_mode_v_op: vk::SamplerAddressMode,
        address_mode_w_op: vk::SamplerAddressMode,
        anisotropy_enable_stp: bool,
        max_anisotropy_stp: f32,
    ) -> ModulResult<Self> {
        match mag_filter_op {
            mag_filter_op => {
                let create_info = vk::SamplerCreateInfo::default()
                    .mag_filter(mag_filter_op)
                    .min_filter(min_filter_op)
                    .address_mode_u(address_mode_u_op)
                    .address_mode_v(address_mode_v_op)
                    .address_mode_w(address_mode_w_op)
                    .anisotropy_enable(anisotropy_enable_stp)
                    .max_anisotropy(max_anisotropy_stp)
                    .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
                    .unnormalized_coordinates(false)
                    .compare_enable(false)
                    .mipmap_mode(vk::SamplerMipmapMode::LINEAR);
                map_vk(unsafe { device_extrl.create_sampler(&create_info, None) })
            }
        }
    }
}
