use ash::vk;

use crate::gpu::MODUL0_VK_FRAME::mem::base::embedded::runtime::frame_info_res_intsct_rt::RenderPassBeginInfoTriangleRt;

/// Catalog — conv/proc entry; Strategy=Triangle; ¬intention (auto branch).
pub trait RenderPassBeginTriangleHandled {
    fn handled_assemble(
        render_pass_extrl: vk::RenderPass,
        framebuffer_extrl: vk::Framebuffer,
        extent_stp: vk::Extent2D,
        clear_color_stp: [f32; 4],
    ) -> RenderPassBeginInfoTriangleRt;
}

trait RenderPassBeginInfoTriangleClearValuesHandled {
    fn handled_assemble(clear_color_stp: [f32; 4]) -> [vk::ClearValue; 2];
}

trait RenderPassBeginInfoTriangleRenderAreaHandled {
    fn handled_assemble(extent_stp: vk::Extent2D) -> vk::Rect2D;
}

impl RenderPassBeginInfoTriangleClearValuesHandled for RenderPassBeginInfoTriangleRt {
    fn handled_assemble(clear_color_stp: [f32; 4]) -> [vk::ClearValue; 2] {
        [
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: clear_color_stp,
                },
            },
            vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            },
        ]
    }
}

impl RenderPassBeginInfoTriangleRenderAreaHandled for RenderPassBeginInfoTriangleRt {
    fn handled_assemble(extent_stp: vk::Extent2D) -> vk::Rect2D {
        vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: extent_stp,
        }
    }
}

impl RenderPassBeginTriangleHandled for RenderPassBeginInfoTriangleRt {
    fn handled_assemble(
        render_pass_extrl: vk::RenderPass,
        framebuffer_extrl: vk::Framebuffer,
        extent_stp: vk::Extent2D,
        clear_color_stp: [f32; 4],
    ) -> RenderPassBeginInfoTriangleRt {
        let clear_values_stp =
            <RenderPassBeginInfoTriangleRt as RenderPassBeginInfoTriangleClearValuesHandled>::handled_assemble(
                clear_color_stp,
            );
        RenderPassBeginInfoTriangleRt {
            render_pass_extrl,
            framebuffer_extrl,
            extent_rt: extent_stp,
            clear_values_rt: clear_values_stp,
            desc: "render_pass_begin_info_triangle",
        }
    }
}

/// `handled_vk_rp_begin` — function (handled vk rp begin).
/// Handled-rank assemble/disassemble entry.
/// Belongs to: frames-in-flight MCG.
#[must_use]
pub fn handled_vk_rp_begin<'a>(
    rt: &'a RenderPassBeginInfoTriangleRt,
) -> vk::RenderPassBeginInfo<'a> {
    let render_area_stp =
        <RenderPassBeginInfoTriangleRt as RenderPassBeginInfoTriangleRenderAreaHandled>::handled_assemble(
            rt.extent_rt,
        );
    vk::RenderPassBeginInfo::default()
        .render_pass(rt.render_pass_extrl)
        .framebuffer(rt.framebuffer_extrl)
        .render_area(render_area_stp)
        .clear_values(&rt.clear_values_rt)
}