use super::{BootstrapLoader};

use anyhow::{anyhow, Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData}
};

#[derive(Debug)]
pub struct SyncObjectsInfo {
    max_frames_in_flight: u32,
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub in_flight_fences: Vec<vk::Fence>,
    pub images_in_flight: Vec<vk::Fence>
}

pub struct SyncFrameSyncObjects {
    pub image_available: vk::Semaphore,
    pub render_finished: vk::Semaphore,
    pub in_flight_fence: vk::Fence
}

impl Default for SyncObjectsInfo {
    fn default() -> Self {
        Self {
            max_frames_in_flight: 2,
            image_available_semaphores: Default::default(),
            render_finished_semaphores: Default::default(),
            in_flight_fences: Default::default(),
            images_in_flight: Default::default()
        }
    }
}

impl SyncObjectsInfo {
    pub fn max_frames_in_flight(&self) -> u32 {
        self.max_frames_in_flight
    }

    pub fn get_sync_objects(&self, sync_frame: usize) -> Result<SyncFrameSyncObjects> {
        if sync_frame as u32 >= self.max_frames_in_flight {
            return Err(anyhow!("Can't get sync objects for sync frame out of range. Only {} frames in flight possible at once!", self.max_frames_in_flight));
        }

        Ok(SyncFrameSyncObjects {
            image_available: self.image_available_semaphores[sync_frame],
            render_finished: self.render_finished_semaphores[sync_frame],
            in_flight_fence: self.in_flight_fences[sync_frame]
        })
    }
}

#[derive(Debug, Default)]
pub struct BootstrapSyncObjectsLoader { }

impl BootstrapSyncObjectsLoader {
    pub fn new() -> Self {
        Self::default()
    }

    fn create_sync_objects(&self, device: &Device, sync_objects_info: &mut SyncObjectsInfo, app_data: &AppData) -> Result<()> {
        let semaphore_info = vk::SemaphoreCreateInfo::builder();
        let fence_info = vk::FenceCreateInfo::builder()
            .flags(vk::FenceCreateFlags::SIGNALED);

        debug!("Creating synchronization objects...");

        let image_available = (0..sync_objects_info.max_frames_in_flight())
            .map(|_| unsafe { device.create_semaphore(&semaphore_info, None) })
            .collect::<Result<Vec<_>, _>>()?;

        let render_finished = (0..sync_objects_info.max_frames_in_flight())
            .map(|_| unsafe { device.create_semaphore(&semaphore_info, None) })
            .collect::<Result<Vec<_>, _>>()?;

        let in_flight_fences = (0..sync_objects_info.max_frames_in_flight())
            .map(|_| unsafe { device.create_fence(&fence_info, None) })
            .collect::<Result<Vec<_>, _>>()?;

        debug!("Synchronization objects created: {:?}, {:?}, {:?}", image_available, render_finished, in_flight_fences);

        sync_objects_info.image_available_semaphores = image_available;
        sync_objects_info.render_finished_semaphores = render_finished;
        sync_objects_info.in_flight_fences = in_flight_fences;

        let image_count = app_data.swapchain.as_ref().unwrap().image_count;
        sync_objects_info.images_in_flight = (0..image_count)
            .map(|_| vk::Fence::null())
            .collect::<Vec<_>>();

        Ok(())
    }

    fn destroy_sync_objects(&self, device: &Device, sync_objects_info: &mut SyncObjectsInfo) -> () {
        debug!("Destroying synchronization semaphores...");

        for semaphore in sync_objects_info.image_available_semaphores.iter() {
            unsafe {
                device.destroy_semaphore(*semaphore, None);
            }
        }
        sync_objects_info.image_available_semaphores.clear();

        for semaphore in sync_objects_info.render_finished_semaphores.iter() {
            unsafe {
                device.destroy_semaphore(*semaphore, None);
            }
        }
        sync_objects_info.render_finished_semaphores.clear();

        for fence in sync_objects_info.in_flight_fences.iter() {
            unsafe {
                device.destroy_fence(*fence, None);
            }
        }
        sync_objects_info.in_flight_fences.clear();
    }
}

impl BootstrapLoader for BootstrapSyncObjectsLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        let mut sync_objects_info = SyncObjectsInfo::default();
        self.create_sync_objects(device, &mut sync_objects_info, app_data)?;
        app_data.sync_objects = Some(sync_objects_info);

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        if let Some(mut sync_objects) = app_data.sync_objects.take() {
            self.destroy_sync_objects(device, &mut sync_objects);
        }
    }

    fn recreate_swapchain(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData, next: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>) -> Result<()> {
        trace!("Recreating nothing in recreate_swapchain");

        next(inst, device, window, app_data)?;

        Ok(())
    }
}
