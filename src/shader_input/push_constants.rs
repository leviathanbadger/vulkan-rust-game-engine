use nalgebra_glm as glm;

use crate::util::{any_as_u8_slice};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct DepthMotionPushConstants {
    pub viewmodel: glm::Mat4,
    pub previous_viewmodel: glm::Mat4
}

impl DepthMotionPushConstants {
    pub fn as_bytes(&self) -> &[u8] {
        let (_, model_bytes, _) = unsafe { any_as_u8_slice(self).align_to::<u8>() };
        model_bytes
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct BaseRenderPushConstants {
    pub viewmodel: glm::Mat4,
    pub normal_viewmodel: glm::Mat4
}

impl BaseRenderPushConstants {
    pub fn as_bytes(&self) -> &[u8] {
        let (_, model_bytes, _) = unsafe { any_as_u8_slice(self).align_to::<u8>() };
        model_bytes
    }
}
