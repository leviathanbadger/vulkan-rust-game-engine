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
    buffer::{Model, CanBeVertexBufferType},
    app_data::{AppData}
};

#[derive(Debug)]
pub struct RenderModelComponent<TVert> where TVert : CanBeVertexBufferType + 'static {
    enabled: bool,
    is_loaded: bool,
    path: &'static str,
    model: Option<Model<TVert>>
}

impl<TVert> RenderModelComponent<TVert> where TVert : CanBeVertexBufferType {
    pub fn new(path: &'static str) -> Result<Self> {
        Ok(Self {
            enabled: true,
            is_loaded: false,
            path: path,
            model: None
        })
    }
}

impl<TVert> CanBeEnabled for RenderModelComponent<TVert> where TVert : CanBeVertexBufferType {
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, enabled: bool) -> () {
        self.enabled = enabled;
    }
}

impl<TVert> GameComponent for RenderModelComponent<TVert> where TVert : CanBeVertexBufferType {
    fn load_and_unload(&mut self, device: &Device, app_data: &AppData) -> Result<()> {
        if self.is_loaded {
            return Ok(());
        }

        let command_pools_info = &app_data.command_pools.as_ref().unwrap();
        let model = Model::<TVert>::new_and_create_from_obj_file(self.path, device, &app_data.memory_properties, command_pools_info)?;
        self.model = Some(model);

        self.is_loaded = true;
        Ok(())
    }

    fn unload(&mut self, device: &Device) -> () {
        if self.is_loaded {
            if let Some(mut model) = self.model.take() {
                model.destroy(device);
            }
            self.is_loaded = false;
        }
    }

    fn render(&self, device: &Device, command_buffer: &vk::CommandBuffer, pipeline_layout: &vk::PipelineLayout, viewmodel: &glm::Mat4, normal_viewmodel: Option<&glm::Mat4>) -> Result<()> {
        let model = self.model.as_ref().unwrap();
        model.write_render_to_command_buffer(device, command_buffer, pipeline_layout, viewmodel, normal_viewmodel)?;

        Ok(())
    }
}
