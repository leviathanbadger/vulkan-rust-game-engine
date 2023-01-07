use nalgebra_glm as glm;

#[derive(Debug, Copy, Clone, Default)]
pub struct DirectionalLight {
    pub direction: glm::Vec3,
    pub color: glm::Vec3
}
