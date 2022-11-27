use super::{
    camera::{Camera, HasCameraMatrix},
    game_object::{GameObject}, can_be_enabled::CanBeEnabled
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

#[derive(Debug, Default)]
pub struct Scene {
    pub render_camera: Camera,
    pub clear_color: glm::Vec3,
    objects: Vec<Box<GameObject>>
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_game_object(&mut self, game_object: Box<GameObject>) -> Result<()> {
        self.objects.push(game_object);

        Ok(())
    }

    pub fn tick(&mut self, frame_info: &FrameInfo) -> Result<()> {
        for obj in self.objects.iter_mut() {
            if obj.is_enabled() {
                obj.tick(frame_info)?;
            }
        }

        Ok(())
    }

    pub fn load_and_unload(&mut self, device: &Device, app_data: &AppData) -> Result<()> {
        for obj in self.objects.iter_mut() {
            if obj.is_enabled() {
                obj.load_and_unload(device, app_data)?;
            }
        }

        Ok(())
    }

    pub fn unload(&mut self, device: &Device) -> () {
        for obj in self.objects.iter_mut() {
            obj.unload(device);
        }
    }

    pub fn render(&self, device: &Device, command_buffer: &vk::CommandBuffer, pipeline_layout: &vk::PipelineLayout) -> Result<()> {
        let view = self.render_camera.get_view_matrix()?;

        for obj in self.objects.iter() {
            if obj.is_enabled() {
                obj.render(device, command_buffer, pipeline_layout, &view)?;
            }
        }

        Ok(())
    }
}
