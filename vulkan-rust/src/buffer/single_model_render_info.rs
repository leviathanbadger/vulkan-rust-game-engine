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
    pub vertex_buffer: vk::Buffer,
    pub vertex_buffer_offset: vk::DeviceSize,
    pub inst_vertex_buffer: Option<vk::Buffer>,
    pub inst_vertex_buffer_offset: vk::DeviceSize,
    pub index_buffer: Option<vk::Buffer>,
    pub index_buffer_offset: vk::DeviceSize,
    pub index_type: vk::IndexType,
    pub element_count: u32,
    pub first_element: u32,
    pub indexed_vertex_offset: i32,

    //TODO: support instanced rendering
    pub instance_count: u32,
    pub first_instance: u32
}

impl SingleModelRenderInfo {
    pub unsafe fn render(&self, device: &Device, command_buffer: &vk::CommandBuffer, pipeline_layout: &vk::PipelineLayout, is_depth_motion_pass: bool) -> Result<()> {
        if let Some(inst_vertex_buffer) = self.inst_vertex_buffer {
            device.cmd_bind_vertex_buffers(*command_buffer, 0, &[self.vertex_buffer, inst_vertex_buffer], &[self.vertex_buffer_offset, self.inst_vertex_buffer_offset]);
        } else {
            device.cmd_bind_vertex_buffers(*command_buffer, 0, &[self.vertex_buffer], &[self.vertex_buffer_offset]);
        }

        if let Some(index_buffer) = self.index_buffer {
            device.cmd_bind_index_buffer(*command_buffer, index_buffer, self.index_buffer_offset, self.index_type);
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

        if let Some(_index_buffer) = self.index_buffer {
            device.cmd_draw_indexed(*command_buffer, self.element_count, self.instance_count, self.first_element, self.indexed_vertex_offset, self.first_instance);
        } else {
            device.cmd_draw(*command_buffer, self.element_count, self.instance_count, self.first_element, self.first_instance);
        }

        Ok(())
    }
}
