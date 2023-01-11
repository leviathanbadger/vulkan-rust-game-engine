use nalgebra_glm as glm;

//Parameters must be aligned specifically. See https://registry.khronos.org/vulkan/specs/1.0-wsi_extensions/html/vkspec.html#interfaces-resources-layout
//Also see https://stackoverflow.com/a/45641579/768597

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct UniformBufferObject {
    //0
    pub proj: glm::Mat4,

    //64
    pub previous_proj: glm::Mat4,

    //128
    pub ambient_light: glm::Vec3,
    #[doc(hidden)]
    pub __pad_ambient_light: u32,

    //144
    pub directional_light_direction: glm::Vec3,
    #[doc(hidden)]
    pub __pad_directional_light_direction: u32,
    //160
    pub directional_light_color: glm::Vec3,
    #[doc(hidden)]
    pub __pad_directional_light_color: u32,

    //176
    pub resolution: glm::Vec2,
    pub jitter: glm::Vec2,

    //192
    pub jitter_scale: f32,
    pub frame_index: u32,
    pub time_in_seconds: f32,
    #[doc(hidden)]
    pub __pad_jitter_scale: u32
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct PostprocessingUniformBufferObject {
    pub frame_index: u32,
    pub time_in_seconds: f32,
    pub exposure: f32
}
