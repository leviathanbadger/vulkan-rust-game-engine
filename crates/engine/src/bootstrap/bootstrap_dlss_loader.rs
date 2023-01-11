use super::{BootstrapLoader};

use std::collections::{HashSet};
use anyhow::{Result};
use dlss_sys::extensions::{get_vulkan_required_extensions};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    bootstrap_loader
};

bootstrap_loader! {
    pub struct BootstrapDlssLoader {
        depends_on();
    }
}

impl BootstrapDlssLoader {

}

impl BootstrapLoader for BootstrapDlssLoader {
    fn add_required_instance_extensions(&self, required_extensions: &mut HashSet<*const i8>) -> Result<()> {
        //Required for integration with DLSS
        required_extensions.insert(vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_EXTENSION.name.as_ptr());

        let dlss_exts = unsafe { get_vulkan_required_extensions(false)? };
        for ext in dlss_exts {
            required_extensions.insert(ext);
        }

        Ok(())
    }

    fn add_required_device_extensions(&self, required_extensions: &mut HashSet<*const i8>) -> Result<()> {
        let dlss_exts = unsafe { get_vulkan_required_extensions(true)? };
        for ext in dlss_exts {
            required_extensions.insert(ext);
        }

        Ok(())
    }
}
