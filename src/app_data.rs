use std::sync::Arc;
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    bootstrap::{
        ValidationInfo,
        SwapchainInfo,
        UniformsInfo,
        RenderImagesInfo,
        PipelineInfo,
        FramebufferInfo,
        CommandPoolsInfo,
        SyncObjectsInfo,
        DescriptorSetInfo
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
    pub render_images: Option<RenderImagesInfo>,
    pub pipeline: Option<PipelineInfo>,
    pub framebuffer: Option<FramebufferInfo>,
    pub command_pools: Option<CommandPoolsInfo>,
    pub sync_objects: Option<SyncObjectsInfo>,
    pub descriptor_sets: Option<DescriptorSetInfo>
}

impl AppData {
    pub fn max_frames_in_flight(&self) -> u32 {
        self.sync_objects.as_ref().unwrap().max_frames_in_flight()
    }
}
