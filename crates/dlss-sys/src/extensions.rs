use std::{ptr, ffi::CStr};
use anyhow::{anyhow, Result};
use crate::dlss;

const BUFFER_DEVICE_ADDRESS_EXT: *const i8 = b"VK_KHR_buffer_device_address\0".as_ptr() as *const i8;

pub unsafe fn get_vulkan_required_extensions(device: bool) -> Result<Vec<*const i8>> {
    let mut instance_count: ::core::ffi::c_uint = 0;
    let mut instance_exts: *mut *const ::core::ffi::c_char = ptr::null_mut();
    let mut device_count: ::core::ffi::c_uint = 0;
    let mut device_exts: *mut *const ::core::ffi::c_char = ptr::null_mut();
    let result = dlss::NVSDK_NGX_VULKAN_RequiredExtensions(&mut instance_count, &mut instance_exts, &mut device_count, &mut device_exts);

    if result != dlss::NVSDK_NGX_Result_NVSDK_NGX_Result_Success {
        return Err(anyhow!("Unexpected error result initializing DLSS: {}", result));
    }

    let capacity = (if device { device_count } else { instance_count }) as usize;
    let mut extensions = Vec::with_capacity(capacity);
    let ext_ptrs = if device { device_exts } else { instance_exts };
    for q in 0..capacity {
        let ptr = *ext_ptrs.offset(q as isize);
        let cstr = CStr::from_ptr(ptr);
        if cstr.to_str().unwrap() == "VK_EXT_buffer_device_address" {
            //Special case, this was added to the vulkan SDK and should be used instead. DLSS supports it as well as the extension they request
            extensions.push(BUFFER_DEVICE_ADDRESS_EXT);
            continue;
        }
        extensions.push(ptr);
    }

    Ok(extensions)
}
