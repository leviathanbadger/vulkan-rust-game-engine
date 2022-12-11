use vulkanalia::{
    prelude::v1_0::*
};

#[derive(Debug, Copy, Clone, Default)]
pub struct Material {
    pub is_loaded: bool,
    pub depth_motion: Option<vk::Pipeline>,
    pub base_render: Option<vk::Pipeline>
}

impl Material {
    pub fn destroy(&mut self, device: &Device) {
        unsafe {
            if let Some(depth_motion) = self.depth_motion {
                device.destroy_pipeline(depth_motion, None);
            }

            if let Some(base_render) = self.base_render {
                device.destroy_pipeline(base_render, None);
            }
        }
    }
}
