use vulkanalia::{
    prelude::v1_0::*
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
    pub swapchain_format: Option<vk::Format>,
    pub swapchain_extent: Option<vk::Extent2D>,
    pub swapchain: Option<vk::SwapchainKHR>,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_image_views: Vec<vk::ImageView>,
    pub render_pass: Option<vk::RenderPass>,
    pub pipeline_layout: Option<vk::PipelineLayout>,
    pub pipeline: Option<vk::Pipeline>,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub command_pool: Option<vk::CommandPool>,
    pub vertex_buffer: Option<vk::Buffer>,
    pub vertex_buffer_memory: Option<vk::DeviceMemory>,
    pub command_buffers: Vec<vk::CommandBuffer>,
    max_frames_in_flight: u32,
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub in_flight_fences: Vec<vk::Fence>,
    pub images_in_flight: Vec<vk::Fence>
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            max_frames_in_flight: 2,

            messenger: Default::default(),
            physical_device: Default::default(),
            memory_properties: Default::default(),
            graphics_queue: Default::default(),
            present_queue: Default::default(),
            graphics_queue_family: Default::default(),
            present_queue_family: Default::default(),
            surface: Default::default(),
            swapchain_format: Default::default(),
            swapchain_extent: Default::default(),
            swapchain: Default::default(),
            swapchain_images: Default::default(),
            swapchain_image_views: Default::default(),
            render_pass: Default::default(),
            pipeline_layout: Default::default(),
            pipeline: Default::default(),
            framebuffers: Default::default(),
            command_pool: Default::default(),
            vertex_buffer: Default::default(),
            vertex_buffer_memory: Default::default(),
            command_buffers: Default::default(),
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
