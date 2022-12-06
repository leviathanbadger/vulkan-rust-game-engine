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
    buffer::{Model, SingleFrameRenderInfo},
    app_data::{AppData},
    shader_input::marble::{MARBLE_INSTANCES, self}
};

#[derive(Debug)]
pub struct RenderMarbleComponent {
    enabled: bool,
    is_loaded: bool,
    path: &'static str,
    model: Option<Model<marble::Vertex, marble::MarbleInstance>>
}

impl RenderMarbleComponent {
    pub fn new(path: &'static str) -> Result<Self> {
        Ok(Self {
            enabled: true,
            is_loaded: false,
            path: path,
            model: None
        })
    }
}

impl CanBeEnabled for RenderMarbleComponent {
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, enabled: bool) -> () {
        self.enabled = enabled;
    }
}

impl GameComponent for RenderMarbleComponent {
    fn load_and_unload(&mut self, device: &Device, app_data: &AppData) -> Result<()> {
        if self.is_loaded {
            return Ok(());
        }

        let command_pools_info = &app_data.command_pools.as_ref().unwrap();
        let model = Model::new_and_create_from_obj_file_instanced(self.path, device, &app_data.memory_properties, command_pools_info, &*MARBLE_INSTANCES)?;
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

    fn create_frame_render_info(&self, frame_info: &mut SingleFrameRenderInfo, viewmodel: &glm::Mat4, previous_viewmodel: Option<&glm::Mat4>) -> Result<()> {
        if let Some(model) = self.model.as_ref() {
            model.create_frame_render_info(frame_info, false, true, viewmodel, previous_viewmodel)?;
        }

        Ok(())
    }
}
