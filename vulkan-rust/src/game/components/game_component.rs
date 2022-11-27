use std::fmt::{Debug};
use nalgebra_glm as glm;
use anyhow::{Result};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    frame_info::{FrameInfo},
    game::{
        can_be_enabled::{CanBeEnabled},
        transform::{Transform}
    },
    app_data::{AppData}
};

pub trait GameComponent : Debug + CanBeEnabled {
    fn tick(&mut self, _frame_info: &FrameInfo, _transform: &mut Transform) -> Result<()> {
        Ok(())
    }

    fn load_and_unload(&mut self, _device: &Device, _app_data: &AppData) -> Result<()> {
        Ok(())
    }
    fn unload(&mut self, _device: &Device) -> () { }

    fn render(&self, _device: &Device, _command_buffer: &vk::CommandBuffer, _pipeline_layout: &vk::PipelineLayout, _viewmodel: &glm::Mat4) -> Result<()> {
        Ok(())
    }
}
