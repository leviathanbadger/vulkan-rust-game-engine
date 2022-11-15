use anyhow::{Result};
use vulkanalia::{
    prelude::v1_0::*,
    vk::{KhrSurfaceExtension}
};

use crate::app::{AppData};

#[derive(Copy, Clone, Debug)]
pub struct QueueFamilyIndices {
    pub graphics: Option<u32>,
    pub present: Option<u32>,
    // pub compute: Option<u32>,
    // pub transfer: Option<u32>,
    // pub sparse_binding: Option<u32>,
    // pub protected: Option<u32>
}

impl QueueFamilyIndices {
    pub unsafe fn get(inst: &Instance, app_data: &AppData, physical_device: vk::PhysicalDevice) -> Result<Self> {
        let properties = inst.get_physical_device_queue_family_properties(physical_device);

        let graphics = properties.iter().position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS)).map(|i| i as u32);
        // let compute = properties.iter().position(|p| p.queue_flags.contains(vk::QueueFlags::COMPUTE)).map(|i| i as u32);
        // let transfer = properties.iter().position(|p| p.queue_flags.contains(vk::QueueFlags::TRANSFER)).map(|i| i as u32);
        // let sparse_binding = properties.iter().position(|p| p.queue_flags.contains(vk::QueueFlags::SPARSE_BINDING)).map(|i| i as u32);
        // let protected = properties.iter().position(|p| p.queue_flags.contains(vk::QueueFlags::PROTECTED)).map(|i| i as u32);

        let mut present = None;
        if let Some(surface) = app_data.surface {
            for (index, _) in properties.iter().enumerate() {
                if let Ok(_) = inst.get_physical_device_surface_support_khr(physical_device, index as u32, surface) {
                    present = Some(index as u32);
                    break;
                }
            }
        }

        Ok(Self {
            graphics,
            present,
            // compute,
            // transfer,
            // sparse_binding,
            // protected
        })
    }
}
