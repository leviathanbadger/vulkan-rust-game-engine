use super::{Buffer, IntoBufferData};
use anyhow::{anyhow, Result};
use nalgebra_glm as glm;
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    shader_input::push_constants::{PushConstants},
    bootstrap::bootstrap_command_buffer_loader::{CommandPoolsInfo}
};

pub trait CanBeIndexBufferType : Copy + Clone {
    fn get_index_type() -> vk::IndexType;
}

impl CanBeIndexBufferType for u16 {
    fn get_index_type() -> vk::IndexType {
        vk::IndexType::UINT16
    }
}

impl CanBeIndexBufferType for u32 {
    fn get_index_type() -> vk::IndexType {
        vk::IndexType::UINT32
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Model<TVert, TIndex = u16> where TVert : Copy + Clone, TIndex : CanBeIndexBufferType {
    vertex_buffer: Buffer<TVert>,
    index_buffer: Buffer<TIndex>,
    require_submit: bool
}

impl<TVert, TIndex> Model<TVert, TIndex> where TVert : Copy + Clone, TIndex : CanBeIndexBufferType {
    pub fn new(max_vertex_count: usize, max_index_count: usize, require_submit: bool) -> Result<Self> {
        if TIndex::get_index_type() == vk::IndexType::UINT8_EXT && max_index_count > u8::MAX as usize {
            return Err(anyhow!("Can't create model with u8 index type with more than {} indices", u8::MAX));
        }
        if TIndex::get_index_type() == vk::IndexType::UINT16 && max_index_count > u16::MAX as usize {
            return Err(anyhow!("Can't create model with u16 index type with more than {} indices", u16::MAX));
        }
        if max_index_count > u32::MAX as usize {
            return Err(anyhow!("Can't create model with more than {} indices", u32::MAX));
        }

        Ok(Self {
            vertex_buffer: Buffer::<TVert>::new(vk::BufferUsageFlags::VERTEX_BUFFER, max_vertex_count, require_submit),
            index_buffer: Buffer::<TIndex>::new(vk::BufferUsageFlags::INDEX_BUFFER, max_index_count, require_submit),
            require_submit
        })
    }

    #[allow(unused)]
    pub fn vertex_buffer(&self) -> &Buffer<TVert> {
        &self.vertex_buffer
    }

    #[allow(unused)]
    pub fn index_buffer(&self) -> &Buffer<TIndex> {
        &self.index_buffer
    }

    pub fn create(&mut self, device: &Device, memory: vk::PhysicalDeviceMemoryProperties) -> Result<()> {
        self.vertex_buffer.create(device, memory)?;
        self.index_buffer.create(device, memory)?;

        Ok(())
    }

    pub fn set_data(&mut self, device: &Device, vertex_data: &impl IntoBufferData<TVert>, inedx_data: &impl IntoBufferData<TIndex>) -> Result<()> {
        self.vertex_buffer.set_data(device, vertex_data)?;
        self.index_buffer.set_data(device, inedx_data)?;

        Ok(())
    }

    pub fn submit(&self, device: &Device, command_pools: &CommandPoolsInfo) -> Result<()> {
        if !self.require_submit {
            warn!("Model submitted that doesn't require data to be submitted.");
            return Ok(());
        }

        command_pools.submit_command_transient_sync(device, |command_buffer| {
            self.write_submit_to_command_buffer(device, command_buffer)
        })
    }

    pub fn write_submit_to_command_buffer(&self, device: &Device, command_buffer: &vk::CommandBuffer) -> Result<()> {
        self.vertex_buffer.write_submit_to_command_buffer(device, command_buffer)?;
        self.index_buffer.write_submit_to_command_buffer(device, command_buffer)?;

        Ok(())
    }

    pub fn write_render_to_command_buffer(&self, device: &Device, command_buffer: &vk::CommandBuffer, pipeline_layout: &vk::PipelineLayout, viewmodel: &glm::Mat4) -> Result<()> {
        unsafe {
            let vertex_buffer = self.vertex_buffer.raw_buffer().unwrap();
            device.cmd_bind_vertex_buffers(*command_buffer, 0, &[vertex_buffer], &[0]);

            let index_buffer = self.index_buffer.raw_buffer().unwrap();
            device.cmd_bind_index_buffer(*command_buffer, index_buffer, 0, TIndex::get_index_type());

            let push_constants = PushConstants {
                viewmodel: *viewmodel
            };
            let push_constants_bytes = push_constants.as_bytes();
            device.cmd_push_constants(*command_buffer, *pipeline_layout, vk::ShaderStageFlags::ALL_GRAPHICS, 0, push_constants_bytes);

            device.cmd_draw_indexed(*command_buffer, self.index_buffer.used_element_count() as u32, 1, 0, 0, 0);
        }

        Ok(())
    }

    pub fn destroy(&mut self, device: &Device) {
        self.vertex_buffer.destroy(device);
        self.index_buffer.destroy(device);
    }
}
