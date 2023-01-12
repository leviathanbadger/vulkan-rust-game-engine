use std::mem::{size_of};

pub unsafe fn any_as_u8_slice<T: Sized>(obj: &T) -> &[u8] {
    std::slice::from_raw_parts((obj as *const T) as *const u8, size_of::<T>())
}
