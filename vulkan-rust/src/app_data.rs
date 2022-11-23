use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    bootstrap::{
        bootstrap_swapchain_loader::{SwapchainInfo},
        bootstrap_uniform_loader::{UniformsInfo},
        bootstrap_depth_buffer_loader::{DepthBufferInfo},
        bootstrap_pipeline_loader::{PipelineInfo},
        bootstrap_command_buffer_loader::{CommandPoolsInfo}
    }
};

#[derive(Debug)]
pub struct AppData {
    pub messenger: Option<vk::DebugUtilsMessengerEXT>,
    pub physical_device: Option<vk::PhysicalDevice>,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub graphics_queue: Option<vk::Queue>,
    pub present_queue: Option<vk::Queue>,
    pub graphics_queue_family: Option<u32>,
    pub present_queue_family: Option<u32>,
    pub surface: Option<vk::SurfaceKHR>,

    pub swapchain: Option<SwapchainInfo>,
    pub uniforms: Option<UniformsInfo>,
    pub depth_buffer: Option<DepthBufferInfo>,
    pub pipeline: Option<PipelineInfo>,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub command_pools: Option<CommandPoolsInfo>,

    max_frames_in_flight: u32,
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub in_flight_fences: Vec<vk::Fence>,
    pub images_in_flight: Vec<vk::Fence>
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            messenger: Default::default(),
            physical_device: Default::default(),
            memory_properties: Default::default(),
            graphics_queue: Default::default(),
            present_queue: Default::default(),
            graphics_queue_family: Default::default(),
            present_queue_family: Default::default(),
            surface: Default::default(),

            swapchain: Default::default(),
            uniforms: Default::default(),
            depth_buffer: Default::default(),
            pipeline: Default::default(),
            framebuffers: Default::default(),
            command_pools: Default::default(),

            max_frames_in_flight: 2,
            image_available_semaphores: Default::default(),
            render_finished_semaphores: Default::default(),
            in_flight_fences: Default::default(),
            images_in_flight: Default::default()
        }
    }
}

impl AppData {
    pub fn max_frames_in_flight(&self) -> u32 {
        self.max_frames_in_flight
    }
}
