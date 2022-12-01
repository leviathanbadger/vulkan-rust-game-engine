use nalgebra_glm as glm;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct UniformBufferObject {
    pub proj: glm::Mat4,

    pub ambient_light: glm::Vec3,
    #[doc(hidden)]
    pub __pad_ambient_light: u32,

    pub directional_light_direction: glm::Vec3,
    #[doc(hidden)]
    pub __pad_directional_light_direction: u32,
    pub directional_light_color: glm::Vec3,
    #[doc(hidden)]
    pub __pad_directional_light_color: u32,

    pub frame_index: u32,
    pub time_in_seconds: f32
}
