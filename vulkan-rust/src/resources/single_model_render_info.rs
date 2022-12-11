use super::{
    MaterialRef,
    ModelRef,
    ResourceLoader,
    model::{HasModelDetails, ReadonlyModel}
};

use nalgebra_glm as glm;
use vulkanalia::{
    prelude::v1_0::*
};
use anyhow::{Result};

use crate::shader_input::push_constants::{DepthMotionPushConstants, BaseRenderPushConstants};

#[derive(Debug, Copy, Clone, Default)]
pub struct SingleModelRenderInfo {
    pub is_static: bool,
    pub is_opaque: bool,
    pub viewmodel: glm::Mat4,
    pub previous_viewmodel: glm::Mat4,

    pub material: MaterialRef,
    pub model: ModelRef,

    pub inst_vertex_buffer: Option<vk::Buffer>,
    pub inst_vertex_buffer_offset: vk::DeviceSize,
    pub first_element: u32,
    pub indexed_vertex_offset: i32,

    //TODO: support instanced rendering
    pub instance_count: u32,
    pub first_instance: u32
}

impl SingleModelRenderInfo {
    fn get_model_details_from_model_ref(&self, model: ModelRef, resource_loader: &ResourceLoader) -> Result<Option<(vk::Buffer, vk::DeviceSize, Option<vk::Buffer>, vk::DeviceSize, vk::IndexType, u32)>> {
        let render_model: Option<ReadonlyModel> = resource_loader.get_render_model(model);

        if let Some(render_model) = render_model {
            let model_details = render_model.get_model_details()?;
            Ok(Some(model_details))
        } else {
            Ok(None)
        }
    }

    pub unsafe fn render(&self, device: &Device, command_buffer: &vk::CommandBuffer, pipeline_layout: &vk::PipelineLayout, is_depth_motion_pass: bool, resource_loader: &ResourceLoader) -> Result<()> {
        let details = self.get_model_details_from_model_ref(self.model, resource_loader)?;
        if details.is_none() {
            return Ok(());
        }
        let (vertex_buffer, vertex_buffer_offset, index_buffer, index_buffer_offset, index_type, element_count) = details.unwrap();

        if let Some(inst_vertex_buffer) = self.inst_vertex_buffer {
            device.cmd_bind_vertex_buffers(*command_buffer, 0, &[vertex_buffer, inst_vertex_buffer], &[vertex_buffer_offset, self.inst_vertex_buffer_offset]);
        } else {
            device.cmd_bind_vertex_buffers(*command_buffer, 0, &[vertex_buffer], &[vertex_buffer_offset]);
        }

        if let Some(index_buffer) = index_buffer {
            device.cmd_bind_index_buffer(*command_buffer, index_buffer, index_buffer_offset, index_type);
        }

        let dm_push_constants: DepthMotionPushConstants;
        let br_push_constants: BaseRenderPushConstants;
        let push_constants_bytes: &[u8];

        if is_depth_motion_pass {
            dm_push_constants = DepthMotionPushConstants {
                viewmodel: self.viewmodel,
                previous_viewmodel: self.previous_viewmodel
            };
            push_constants_bytes = dm_push_constants.as_bytes();
        } else {
            br_push_constants = BaseRenderPushConstants {
                viewmodel: self.viewmodel,
                normal_viewmodel: glm::transpose(&glm::inverse(&self.viewmodel))
            };
            push_constants_bytes = br_push_constants.as_bytes();
        }

        device.cmd_push_constants(*command_buffer, *pipeline_layout, vk::ShaderStageFlags::ALL_GRAPHICS, 0, push_constants_bytes);

        if let Some(_index_buffer) = index_buffer {
            device.cmd_draw_indexed(*command_buffer, element_count, self.instance_count, self.first_element, self.indexed_vertex_offset, self.first_instance);
        } else {
            device.cmd_draw(*command_buffer, element_count, self.instance_count, self.first_element, self.first_instance);
        }

        Ok(())
    }
}
