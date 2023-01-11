use super::{
    camera::{Camera, HasCameraMatrix},
    game_object::{GameObject},
    can_be_enabled::{CanBeEnabled},
    lights::{DirectionalLight}
};

use nalgebra_glm as glm;
use anyhow::{Result};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    frame_info::{FrameInfo},
    resources::{SingleFrameRenderInfo, ResourceLoader}
};

#[derive(Debug, Default)]
pub struct Scene {
    pub render_camera: Camera,
    pub clear_color: glm::Vec3,
    pub ambient_light: glm::Vec3,
    pub directional_light: Option<DirectionalLight>,
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

    pub fn load_and_unload(&mut self, resource_loader: &mut ResourceLoader) -> Result<()> {
        for obj in self.objects.iter_mut() {
            if obj.is_enabled() {
                obj.load_and_unload(resource_loader)?;
            }
        }

        Ok(())
    }

    pub fn unload(&mut self, resource_loader: &mut ResourceLoader) -> () {
        for obj in self.objects.iter_mut() {
            obj.unload(resource_loader);
        }
    }

    pub fn create_frame_render_info(&self, frame_info: &mut SingleFrameRenderInfo, bounds: vk::Extent2D) -> Result<()> {
        let projection = self.render_camera.get_projection_matrix(bounds)?;
        let previous_projection = *self.render_camera.get_previous_projection_matrix().unwrap_or(&projection);
        let view = self.render_camera.get_view_matrix()?;

        frame_info.proj = projection;
        frame_info.previous_proj = previous_projection;

        frame_info.clear_color = self.clear_color;

        frame_info.ambient_light = self.ambient_light;

        if let Some(directional_light) = self.directional_light {
            let dir = directional_light.direction.normalize();
            let normal_matrix = glm::convert::<glm::DMat4, glm::Mat4>(glm::transpose(&glm::inverse(&view)));
            let actual_direction: glm::Vec3 = (normal_matrix * glm::vec4(dir.x, dir.y, dir.z, 0.0)).xyz().normalize();

            frame_info.directional_light_color = directional_light.color;
            frame_info.directional_light_direction = actual_direction;
        }

        for obj in self.objects.iter() {
            if obj.is_enabled() {
                obj.create_frame_render_info(frame_info, &view)?;
            }
        }

        Ok(())
    }

    pub fn end_frame(&mut self, bounds: vk::Extent2D) -> Result<()> {
        self.render_camera.end_frame(bounds)?;
        let view = self.render_camera.get_previous_view_matrix().unwrap();

        for obj in self.objects.iter_mut() {
            if obj.is_enabled() {
                obj.end_frame(&view)?;
            }
        }

        Ok(())
    }
}
