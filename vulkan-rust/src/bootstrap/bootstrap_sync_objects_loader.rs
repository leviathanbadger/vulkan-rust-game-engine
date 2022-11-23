use super::{BootstrapLoader};

use anyhow::{Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData}
};

#[derive(Debug, Default)]
pub struct BootstrapSyncObjectsLoader { }

impl BootstrapSyncObjectsLoader {
    pub fn new() -> Self {
        Self::default()
    }

    fn create_sync_objects(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        let semaphore_info = vk::SemaphoreCreateInfo::builder();
        let fence_info = vk::FenceCreateInfo::builder()
            .flags(vk::FenceCreateFlags::SIGNALED);

        debug!("Creating synchronization objects...");

        let image_available = (0..app_data.max_frames_in_flight())
            .map(|_| unsafe { device.create_semaphore(&semaphore_info, None) })
            .collect::<Result<Vec<_>, _>>()?;

        let render_finished = (0..app_data.max_frames_in_flight())
            .map(|_| unsafe { device.create_semaphore(&semaphore_info, None) })
            .collect::<Result<Vec<_>, _>>()?;

        let in_flight_fences = (0..app_data.max_frames_in_flight())
            .map(|_| unsafe { device.create_fence(&fence_info, None) })
            .collect::<Result<Vec<_>, _>>()?;

        debug!("Synchronization objects created: {:?}, {:?}, {:?}", image_available, render_finished, in_flight_fences);

        app_data.image_available_semaphores = image_available;
        app_data.render_finished_semaphores = render_finished;
        app_data.in_flight_fences = in_flight_fences;

        let image_count = app_data.swapchain.as_ref().unwrap().image_count;
        app_data.images_in_flight = (0..image_count)
            .map(|_| vk::Fence::null())
            .collect::<Vec<_>>();

        Ok(())
    }

    fn destroy_sync_objects(&self, device: &Device, app_data: &mut AppData) -> () {
        debug!("Destroying synchronization semaphores...");

        for semaphore in app_data.image_available_semaphores.iter() {
            unsafe {
                device.destroy_semaphore(*semaphore, None);
            }
        }
        app_data.image_available_semaphores.clear();

        for semaphore in app_data.render_finished_semaphores.iter() {
            unsafe {
                device.destroy_semaphore(*semaphore, None);
            }
        }
        app_data.render_finished_semaphores.clear();

        for fence in app_data.in_flight_fences.iter() {
            unsafe {
                device.destroy_fence(*fence, None);
            }
        }
        app_data.in_flight_fences.clear();
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

    fn recreate_swapchain(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData, next: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>) -> Result<()> {
        trace!("Recreating nothing in recreate_swapchain");

        next(inst, device, window, app_data)?;

        Ok(())
    }
}
