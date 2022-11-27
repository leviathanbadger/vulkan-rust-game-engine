use super::{GameComponent};

use nalgebra_glm as glm;
use anyhow::{Result};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    game::{
        can_be_enabled::{CanBeEnabled}
    },
    buffer::{Model, CanBeIndexBufferType},
    app_data::{AppData}
};

#[derive(Debug)]
pub struct RenderModelComponent<TVert, TIndex = u16> where TVert : Copy + Clone + ::std::fmt::Debug + 'static, TIndex : CanBeIndexBufferType + 'static {
    enabled: bool,
    is_loaded: bool,
    model: Model<TVert, TIndex>,
    vertices: &'static Vec<TVert>,
    indices: &'static Vec<TIndex>
}

impl<TVert, TIndex> RenderModelComponent<TVert, TIndex> where TVert : Copy + Clone + ::std::fmt::Debug, TIndex : CanBeIndexBufferType {
    pub fn new(vertices: &'static Vec<TVert>, indices: &'static Vec<TIndex>) -> Result<Self> {
        let model = Model::new(vertices.len(), indices.len(), true)?;

        Ok(Self {
            enabled: true,
            is_loaded: false,
            model,
            vertices,
            indices
        })
    }
}

impl<TVert, TIndex> CanBeEnabled for RenderModelComponent<TVert, TIndex> where TVert : Copy + Clone + ::std::fmt::Debug, TIndex : CanBeIndexBufferType {
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, enabled: bool) -> () {
        self.enabled = enabled;
    }
}

impl<TVert, TIndex> GameComponent for RenderModelComponent<TVert, TIndex> where TVert : Copy + Clone + ::std::fmt::Debug, TIndex : CanBeIndexBufferType {
    fn load_and_unload(&mut self, device: &Device, app_data: &AppData) -> Result<()> {
        if self.is_loaded {
            return Ok(());
        }
        self.is_loaded = true;

        let command_pools_info = &app_data.command_pools.as_ref().unwrap();

        self.model.create(device, app_data.memory_properties)?;
        self.model.set_data(device, self.vertices, self.indices)?;
        self.model.submit(device, command_pools_info)?;

        Ok(())
    }

    fn unload(&mut self, device: &Device) -> () {
        if self.is_loaded {
            self.model.destroy(device);
            self.is_loaded = false;
        }
    }

    fn render(&self, device: &Device, command_buffer: &vk::CommandBuffer, pipeline_layout: &vk::PipelineLayout, viewmodel: &glm::Mat4) -> Result<()> {
        self.model.write_render_to_command_buffer(device, command_buffer, pipeline_layout, viewmodel)?;

        Ok(())
    }
}
