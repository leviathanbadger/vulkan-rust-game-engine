use nalgebra_glm as glm;

use crate::util::{any_as_u8_slice};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct PushConstants {
    pub viewmodel: glm::Mat4,
    pub normal_viewmodel: glm::Mat4,
    pub previous_viewmodel: glm::Mat4
}

impl PushConstants {
    pub fn as_bytes(&self) -> &[u8] {
        let (_, model_bytes, _) = unsafe { any_as_u8_slice(self).align_to::<u8>() };
        model_bytes
    }
}
