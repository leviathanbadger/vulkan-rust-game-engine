use super::{GameComponent};

use nalgebra_glm as glm;
use anyhow::{Result};

use crate::{
    frame_info::{FrameInfo},
    game::{
        can_be_enabled::{CanBeEnabled},
        transform::{Transform, DEFAULT_UP}
    }
};

#[derive(Debug, Copy, Clone)]
pub struct RotateOverTimeComponent {
    enabled: bool,
    pub time: f32,
    pub angle: f32
}

impl RotateOverTimeComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for RotateOverTimeComponent {
    fn default() -> Self {
        Self {
            enabled: true,
            time: 0.0,
            angle: 0.0
        }
    }
}

impl CanBeEnabled for RotateOverTimeComponent {
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    fn set_enabled(&mut self, enabled: bool) -> () {
        self.enabled = enabled;
    }
}

impl GameComponent for RotateOverTimeComponent {
    fn tick(&mut self, frame_info: &FrameInfo, transform: &mut Transform) -> Result<()> {
        self.time += frame_info.current_frame_delta_time.as_secs_f32();
        self.angle = self.time * -glm::radians(&glm::vec1(90.0))[0]; // Rotate 90 degrees per second

        let rotate_matrix = glm::rotate(&glm::identity(), self.angle, &*DEFAULT_UP);
        transform.orient = glm::to_quat(&rotate_matrix);

        Ok(())
    }
}
