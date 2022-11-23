use super::BootstrapLoader;

use std::{os::raw::c_void, ffi::CStr};
use anyhow::{Result};
use vulkanalia::{
    prelude::v1_0::*,
    vk::{InstanceCreateInfoBuilder, ExtDebugUtilsExtension}
};

use crate::{
    app_data::{AppData}
};

const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

#[derive(Debug, Default)]
pub struct ValidationInfo {
    pub messenger: vk::DebugUtilsMessengerEXT
}

#[derive(Debug, Default)]
pub struct BootstrapValidationLoader { }

impl BootstrapValidationLoader {
    pub fn new() -> Self {
        Self::default()
    }
}

extern "system" fn debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _type: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void
) -> vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        error!("[Vulkan DebugUtils: {:?}] {}", _type, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        warn!("[Vulkan DebugUtils: {:?}] {}", _type, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        debug!("[Vulkan DebugUtils: {:?}] {}", _type, message);
    } else {
        trace!("[Vulkan DebugUtils: {:?}] {}", _type, message);
    }

    vk::FALSE
}

impl BootstrapLoader for BootstrapValidationLoader {
    fn add_required_instance_layers(&self, required_layers: &mut Vec<*const i8>) -> Result<()> {
        required_layers.push(VALIDATION_LAYER.as_ptr());

        Ok(())
    }

    fn add_required_instance_extensions(&self, required_extensions: &mut Vec<*const i8>) -> Result<()> {
        required_extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());

        Ok(())
    }

    fn instance_create(&self, inst_info: InstanceCreateInfoBuilder, app_data: &mut AppData, next: &dyn Fn(InstanceCreateInfoBuilder, &mut AppData) -> Result<Instance>) -> Result<Instance> {
        debug!("Adding Vulkan debug utils messenger to instance create.");

        let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
            .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
            .user_callback(Some(debug_callback));
        trace!("debug_info: {:?}", debug_info);

        let inst = next(inst_info.push_next(&mut debug_info), app_data)?;

        debug!("Creating Vulkan debug utils messenger. Future validation/error/diagnostics from Vulkan will be logged.");
        let messenger: vk::DebugUtilsMessengerEXT;
        unsafe {
            messenger = inst.create_debug_utils_messenger_ext(&debug_info, None)?;
        }

        app_data.validation = Some(ValidationInfo { messenger });

        Ok(inst)
    }

    fn before_destroy_instance(&self, inst: &Instance, app_data: &mut AppData) -> () {
        if let Some(validation) = app_data.validation.take() {
            debug!("Destroying Vulkan debug utils messenger. Additional Vulkan messages may not be logged");
            unsafe {
                inst.destroy_debug_utils_messenger_ext(validation.messenger, None);
            }
        }
    }

    fn add_required_device_layers(&self, required_layers: &mut Vec<*const i8>) -> Result<()> {
        required_layers.push(VALIDATION_LAYER.as_ptr());

        Ok(())
    }
}
