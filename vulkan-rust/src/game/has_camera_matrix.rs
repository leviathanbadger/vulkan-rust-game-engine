use nalgebra_glm as glm;
use anyhow::{Result};
use vulkanalia::{
    prelude::v1_0::*
};

pub trait HasCameraMatrix {
    fn get_view_matrix(&self) -> Result<glm::Mat4>;
    fn get_projection_matrix(&self, bounds: vk::Extent2D) -> Result<glm::Mat4>;
}
