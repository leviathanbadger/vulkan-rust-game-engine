use super::{get_memory_type_index};
use anyhow::{anyhow, Result};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::app_data::{AppData};

#[derive(Debug, Copy, Clone)]
pub struct Image2D {
    pub format: Option<vk::Format>,
    pub size: Option<vk::Extent2D>,
    pub image: Option<vk::Image>,
    pub image_memory: Option<vk::DeviceMemory>,
    pub image_view: Option<vk::ImageView>,
    initialized: bool,
    owns_image: bool
}

impl Default for Image2D {
    fn default() -> Self {
        Self {
            format: Default::default(),
            size: Default::default(),
            image: Default::default(),
            image_memory: Default::default(),
            image_view: Default::default(),
            initialized: false,
            owns_image: true
        }
    }
}

impl Image2D {
    pub fn new() -> Self {
        Self::default()
    }
    fn new_with_image(image: vk::Image, format: vk::Format, size: vk::Extent2D) -> Self {
        Self {
            format: Some(format),
            size: Some(size),
            image: Some(image),
            owns_image: false,
            ..Self::default()
        }
    }

    unsafe fn get_supported_format(&self, inst: &Instance, app_data: &AppData, candidates: &[vk::Format], tiling: vk::ImageTiling, features: vk::FormatFeatureFlags) -> Result<vk::Format> {
        let physical_device = app_data.physical_device.unwrap();
        for format in candidates.iter() {
            let properties = inst.get_physical_device_format_properties(physical_device, *format);
            let supported = match tiling {
                vk::ImageTiling::LINEAR => properties.linear_tiling_features.contains(features),
                vk::ImageTiling::OPTIMAL => properties.optimal_tiling_features.contains(features),
                _ => false
            };
            if supported {
                return Ok(*format);
            }
        }

        Err(anyhow!("Failed to find a supported format for depth/stencil buffer"))
    }

    unsafe fn choose_depth_stencil_format(&self, inst: &Instance, app_data: &AppData) -> Result<vk::Format> {
        let candidates = &[
            vk::Format::D32_SFLOAT_S8_UINT,
            vk::Format::D24_UNORM_S8_UINT,
            vk::Format::D32_SFLOAT
        ];

        self.get_supported_format(inst, app_data, candidates, vk::ImageTiling::OPTIMAL, vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT)
    }

    fn create_image(&mut self, device: &Device, app_data: &AppData, size: vk::Extent2D, format: vk::Format, tiling: vk::ImageTiling, usage_flags: vk::ImageUsageFlags, memory_flags: vk::MemoryPropertyFlags) -> Result<()> {
        let image_info = vk::ImageCreateInfo::builder()
            .image_type(vk::ImageType::_2D)
            .extent(vk::Extent3D { width: size.width, height: size.height, depth: 1 })
            .mip_levels(1)
            .array_layers(1)
            .format(format)
            .tiling(tiling)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .usage(usage_flags)
            .samples(vk::SampleCountFlags::_1)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let image: vk::Image;
        let requirements: vk::MemoryRequirements;
        unsafe {
            image = device.create_image(&image_info, None)?;
            requirements = device.get_image_memory_requirements(image);
        }

        let memory_type_index = get_memory_type_index(app_data.memory_properties, memory_flags, requirements)?;
        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(memory_type_index);

        let image_memory: vk::DeviceMemory;
        unsafe {
            image_memory = device.allocate_memory(&alloc_info, None)?;
            device.bind_image_memory(image, image_memory, 0)?;
        }

        self.image = Some(image);
        self.image_memory = Some(image_memory);

        Ok(())
    }

    fn create_image_view(&mut self, device: &Device, aspect_flags: vk::ImageAspectFlags) -> Result<()> {
        let components = vk::ComponentMapping::builder()
            .r(vk::ComponentSwizzle::IDENTITY)
            .g(vk::ComponentSwizzle::IDENTITY)
            .b(vk::ComponentSwizzle::IDENTITY)
            .a(vk::ComponentSwizzle::IDENTITY);

        let subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask(aspect_flags)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1);

        let image_view_info = vk::ImageViewCreateInfo::builder()
            .image(self.image.unwrap())
            .view_type(vk::ImageViewType::_2D)
            .format(self.format.unwrap())
            .components(components)
            .subresource_range(subresource_range);

        let image_view: vk::ImageView;
        unsafe {
            image_view = device.create_image_view(&image_view_info, None)?;
        }

        self.image_view = Some(image_view);

        Ok(())
    }

    pub fn format(&self) -> Option<vk::Format> {
        self.format
    }

    #[allow(unused)]
    pub fn size(&self) -> Option<vk::Extent2D> {
        self.size
    }

    #[allow(unused)]
    pub unsafe fn raw_image(&self) -> Option<vk::Image> {
        self.image
    }

    #[allow(unused)]
    pub unsafe fn raw_image_memory(&self) -> Option<vk::DeviceMemory> {
        self.image_memory
    }

    #[allow(unused)]
    pub unsafe fn raw_image_view(&self) -> Option<vk::ImageView> {
        self.image_view
    }

    pub fn create_depth_stencil_buffer(&mut self, inst: &Instance, device: &Device, app_data: &AppData) -> Result<()> {
        if self.initialized {
            return Err(anyhow!("This image has already been initialized. It can't be created again!"));
        }
        if !self.owns_image {
            return Err(anyhow!("This Image2D was constructed with a passed-in vk::Image. It can't be created in a way that creates a new vk::Image."));
        }

        let format = unsafe { self.choose_depth_stencil_format(inst, app_data)? };
        self.format = Some(format);

        let size = app_data.swapchain.as_ref().unwrap().extent;
        self.size = Some(size);

        self.create_image(device, app_data, size, format, vk::ImageTiling::OPTIMAL, vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT, vk::MemoryPropertyFlags::DEVICE_LOCAL)?;
        self.create_image_view(device, vk::ImageAspectFlags::DEPTH)?;

        self.initialized = true;

        Ok(())
    }

    pub fn create_from_swapchain_images(swapchain_images: &[vk::Image], format: vk::Format, size: vk::Extent2D, device: &Device) -> Result<Vec<Image2D>> {
        swapchain_images.iter()
            .map(|i| {
                let mut image = Self::new_with_image(*i, format, size);
                image.create_from_swapchain_image(device)?;
                Ok(image)
            })
            .collect::<Result<Vec<_>, _>>()
    }
    pub fn create_from_swapchain_image(&mut self, device: &Device) -> Result<()> {
        if self.initialized {
            return Err(anyhow!("This image has already been initialized. It can't be created again!"));
        }
        if self.owns_image {
            return Err(anyhow!("This Image2D was constructed without a passed-in vk::Image. It can't be created in a way that requires a preexisting vk::Image."));
        }

        self.create_image_view(device, vk::ImageAspectFlags::COLOR)?;

        self.initialized = true;

        Ok(())
    }

    pub fn destroy(&mut self, device: &Device) {
        if let Some(image_view) = self.image_view.take() {
            unsafe {
                device.destroy_image_view(image_view, None);
            }
        }

        if self.owns_image {
            if let Some(image) = self.image.take() {
                unsafe {
                    device.destroy_image(image, None);
                }
            }

            if let Some(image_memory) = self.image_memory.take() {
                unsafe {
                    device.free_memory(image_memory, None);
                }
            }

            self.format = None;
            self.size = None;
        }

        self.initialized = false;
    }
}
