use std::sync::Arc;
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    bootstrap::{
        bootstrap_validation_loader::{ValidationInfo},
        bootstrap_swapchain_loader::{SwapchainInfo},
        bootstrap_uniform_loader::{UniformsInfo},
        bootstrap_depth_buffer_loader::{DepthBufferInfo},
        bootstrap_pipeline_loader::{PipelineInfo},
        bootstrap_framebuffer_loader::{FramebufferInfo},
        bootstrap_command_buffer_loader::{CommandPoolsInfo},
        bootstrap_sync_objects_loader::{SyncObjectsInfo}
    }
};

#[derive(Debug, Copy, Clone, Default)]
pub struct VulkanQueueInfo {
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
    pub graphics_queue_family: u32,
    pub present_queue_family: u32
}

#[derive(Debug, Default)]
pub struct AppData {
    pub physical_device: Option<vk::PhysicalDevice>,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub queue_info: Option<Arc<VulkanQueueInfo>>,
    pub surface: Option<vk::SurfaceKHR>,

    pub validation: Option<ValidationInfo>,
    pub swapchain: Option<SwapchainInfo>,
    pub uniforms: Option<UniformsInfo>,
    pub depth_buffer: Option<DepthBufferInfo>,
    pub pipeline: Option<PipelineInfo>,
    pub framebuffer: Option<FramebufferInfo>,
    pub command_pools: Option<CommandPoolsInfo>,
    pub sync_objects: Option<SyncObjectsInfo>
}

impl AppData {
    pub fn max_frames_in_flight(&self) -> u32 {
        self.sync_objects.as_ref().unwrap().max_frames_in_flight()
    }
}
