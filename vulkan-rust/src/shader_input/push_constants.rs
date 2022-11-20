use std::mem::{size_of};
use nalgebra_glm as glm;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct PushConstants {
    pub viewmodel: glm::Mat4
}

unsafe fn any_as_u8_slice<T: Sized>(obj: &T) -> &[u8] {
    std::slice::from_raw_parts((obj as *const T) as *const u8, size_of::<T>())
}

impl PushConstants {
    pub fn as_bytes(&self) -> &[u8] {
        let (_, model_bytes, _) = unsafe { any_as_u8_slice(self).align_to::<u8>() };
        model_bytes
    }
}
