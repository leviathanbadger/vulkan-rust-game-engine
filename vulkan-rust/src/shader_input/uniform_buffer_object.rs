use nalgebra_glm as glm;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct UniformBufferObject {
    pub proj: glm::Mat4,
    pub view: glm::Mat4,
    pub frame_index: u32,
    pub time_in_seconds: f32
}
