use crate::app::{AppData};

use super::{BootstrapLoader};

use anyhow::{Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

#[derive(Debug, Default)]
pub struct BootstrapSyncObjectsLoader { }

impl BootstrapSyncObjectsLoader {
    pub fn new() -> Self {
        Self::default()
    }

    fn create_sync_objects(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        let semaphore_info = vk::SemaphoreCreateInfo::builder();

        let image_available: vk::Semaphore;
        let render_finished: vk::Semaphore;
        unsafe {
            debug!("Creating synchronization semaphores...");
            image_available = device.create_semaphore(&semaphore_info, None)?;
            render_finished = device.create_semaphore(&semaphore_info, None)?;
            debug!("Synchronization semaphores created: {:?}, {:?}", image_available, render_finished);
        }
        app_data.image_available_semaphore = Some(image_available);
        app_data.render_finished_semaphore = Some(render_finished);

        Ok(())
    }

    fn destroy_sync_objects(&self, device: &Device, app_data: &mut AppData) -> () {
        debug!("Destroying synchronization semaphores...");

        if let Some(image_available) = app_data.image_available_semaphore.take() {
            unsafe {
                device.destroy_semaphore(image_available, None);
            }
        }

        if let Some(render_finished) = app_data.render_finished_semaphore.take() {
            unsafe {
                device.destroy_semaphore(render_finished, None);
            }
        }
    }
}

impl BootstrapLoader for BootstrapSyncObjectsLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        self.create_sync_objects(device, app_data)?;

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        self.destroy_sync_objects(device, app_data);
    }
}
