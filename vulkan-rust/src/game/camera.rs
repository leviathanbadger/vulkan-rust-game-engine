use super::transform::{Transform};

use nalgebra_glm as glm;
use anyhow::{Result};
use vulkanalia::{
    prelude::v1_0::*
};

pub use crate::game::has_camera_matrix::{HasCameraMatrix};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum CameraKind {
    #[default]
    Perspective,
    #[allow(unused)]
    Orthographic
}

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub transform: Transform,
    near: f32,
    far: f32,
    kind: CameraKind,
    fovy: f32
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            near: 0.001,
            far: 1000.0,
            kind: Default::default(),
            fovy: 45.0
        }
    }
}

#[allow(unused)]
impl Camera {
    pub fn look_at(&mut self, target: glm::DVec3) -> () {
        self.transform.look_at(target);
    }
    pub fn look_at_up(&mut self, target: glm::DVec3, up: glm::Vec3) -> () {
        self.transform.look_at_up(target, up);
    }

    pub fn near(&self) -> f32 {
        self.near
    }
    pub fn set_near(&mut self, near: f32) -> () {
        self.near = near;
    }

    pub fn far(&self) -> f32 {
        self.far
    }
    pub fn set_far(&mut self, far: f32) -> () {
        self.far = far;
    }

    pub fn kind(&self) -> CameraKind {
        self.kind
    }
    pub fn set_kind(&mut self, kind: CameraKind) -> () {
        self.kind = kind;
    }

    pub fn fovy(&self) -> f32 {
        self.fovy
    }
    pub fn set_fovy(&mut self, fovy: f32) -> () {
        self.fovy = fovy;
    }
}

impl HasCameraMatrix for Camera {
    fn get_view_matrix(&self) -> Result<glm::DMat4> {
        self.transform.as_matrix_inverse()
    }

    fn get_projection_matrix(&self, bounds: vk::Extent2D) -> Result<glm::Mat4> {
        let mut projection = match self.kind {
            CameraKind::Perspective => {
                let aspect_ratio = bounds.width as f32 / bounds.height as f32;
                let fovy = glm::radians(&glm::vec1(self.fovy))[0];
                glm::perspective_lh_zo(aspect_ratio, fovy, self.near, self.far)
            },
            CameraKind::Orthographic => {
                glm::ortho(-(bounds.width as f32) / 2.0, (bounds.width as f32) / 2.0, -(bounds.height as f32) / 2.0, (bounds.height as f32) / 2.0, self.near, self.far)
            }
        };

        projection[(1, 1)] *= -1.0;

        Ok(projection)
    }
}
