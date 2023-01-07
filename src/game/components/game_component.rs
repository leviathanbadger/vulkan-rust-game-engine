use std::fmt::{Debug};
use nalgebra_glm as glm;
use anyhow::{Result};

use crate::{
    frame_info::{FrameInfo},
    game::{
        can_be_enabled::{CanBeEnabled},
        transform::{Transform}
    },
    resources::{SingleFrameRenderInfo, ResourceLoader}
};

pub trait GameComponent : Debug + CanBeEnabled {
    fn tick(&mut self, _frame_info: &FrameInfo, _transform: &mut Transform) -> Result<()> {
        Ok(())
    }

    fn load_and_unload(&mut self, _resource_loader: &mut ResourceLoader) -> Result<()> {
        Ok(())
    }
    fn unload(&mut self, _resource_loader: &mut ResourceLoader) -> () {
    }

    fn create_frame_render_info(&self, _frame_info: &mut SingleFrameRenderInfo, _viewmodel: &glm::Mat4, _previous_viewmodel: Option<&glm::Mat4>) -> Result<()> {
        Ok(())
    }
}
