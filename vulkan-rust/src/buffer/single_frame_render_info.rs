use super::{SingleModelRenderInfo};

use nalgebra_glm as glm;

#[derive(Debug)]
pub struct SingleFrameRenderInfo {
    pub frame_index: u32,
    pub time_in_seconds: f32,

    pub proj: glm::Mat4,
    pub previous_proj: glm::Mat4,

    pub clear_color: glm::Vec3,

    pub ambient_light: glm::Vec3,

    pub directional_light_direction: glm::Vec3,
    pub directional_light_color: glm::Vec3,

    pub models_to_render: Vec<SingleModelRenderInfo>
}

impl Default for SingleFrameRenderInfo {
    fn default() -> Self {
        Self {
            proj: glm::identity(),
            previous_proj: glm::identity(),

            clear_color: Default::default(),

            ambient_light: Default::default(),

            directional_light_direction: glm::vec3(-1.0, 0.0, 0.0),
            directional_light_color: Default::default(),

            frame_index: Default::default(),
            time_in_seconds: Default::default(),

            models_to_render: Vec::with_capacity(200)
        }
    }
}
