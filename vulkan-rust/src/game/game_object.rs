use super::{
    components::{GameComponent},
    can_be_enabled::{CanBeEnabled},
    transform::{Transform}
};

use nalgebra_glm as glm;
use anyhow::{Result};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    frame_info::{FrameInfo},
    app_data::{AppData}
};

#[derive(Debug)]
pub struct GameObject {
    enabled: bool,
    components: Vec<Box<dyn GameComponent>>,
    pub transform: Transform
}

impl Default for GameObject {
    fn default() -> Self {
        Self {
            enabled: true,
            components: Default::default(),
            transform: Default::default()
        }
    }
}

impl CanBeEnabled for GameObject {
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, enabled: bool) -> () {
        self.enabled = enabled;
    }
}

impl GameObject {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_component(&mut self, component: Box<dyn GameComponent>) -> Result<()> {
        self.components.push(component);

        Ok(())
    }

    pub fn tick(&mut self, frame_info: &FrameInfo) -> Result<()> {
        for component in self.components.iter_mut() {
            if component.is_enabled() {
                component.tick(frame_info, &mut self.transform)?;
            }
        }

        Ok(())
    }

    pub fn load_and_unload(&mut self, device: &Device, app_data: &AppData) -> Result<()> {
        for component in self.components.iter_mut() {
            if component.is_enabled() {
                component.load_and_unload(device, app_data)?;
            }
        }

        Ok(())
    }

    pub fn unload(&mut self, device: &Device) -> () {
        for component in self.components.iter_mut() {
            component.unload(device);
        }
    }

    pub fn render(&self, device: &Device, command_buffer: &vk::CommandBuffer, pipeline_layout: &vk::PipelineLayout, view: &glm::DMat4) -> Result<()> {
        let model = self.transform.as_matrix()?;
        let modelview = glm::convert::<glm::DMat4, glm::Mat4>(view * model);

        for component in self.components.iter() {
            if component.is_enabled() {
                component.render(device, command_buffer, pipeline_layout, &modelview)?;
            }
        }

        Ok(())
    }
}
