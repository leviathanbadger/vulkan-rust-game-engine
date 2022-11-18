use super::{BootstrapLoader};

use anyhow::{anyhow, Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*,
    vk::{PhysicalDeviceProperties, PhysicalDeviceFeatures, KhrSurfaceExtension, KhrSwapchainExtension},
};

use crate::{
    app::{GraphicsCardSuitabilityError},
    app_data::{AppData}
};

#[derive(Debug)]
pub struct SwapchainSupport {
    capabilities: vk::SurfaceCapabilitiesKHR,
    formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>
}

impl SwapchainSupport {
    unsafe fn get(inst: &Instance, app_data: &AppData, physical_device: vk::PhysicalDevice) -> Result<Self> {
        if let Some(surface) = app_data.surface {
            Ok(Self {
                capabilities: inst.get_physical_device_surface_capabilities_khr(physical_device, surface)?,
                formats: inst.get_physical_device_surface_formats_khr(physical_device, surface)?,
                present_modes: inst.get_physical_device_surface_present_modes_khr(physical_device, surface)?
            })
        } else {
            Err(anyhow!("Unable to get swapchain support. No KHR surface to check against"))
        }
    }
}

#[derive(Debug, Default)]
pub struct BootstrapSwapchainLoader { }

impl BootstrapSwapchainLoader {
    pub fn new() -> Self {
        Self::default()
    }

    fn choose_surface_format(&self, swapchain_support: &SwapchainSupport) -> Option<vk::SurfaceFormatKHR> {
        let available_formats = &swapchain_support.formats[..];

        for format in available_formats {
            if format.format == vk::Format::B8G8R8A8_SRGB && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR {
                return Some(*format);
            }
        }

        let first = available_formats.first();
        if let Some(format) = first {
            Some(*format)
        } else {
            None
        }
    }

    fn choose_presentation_mode(&self, swapchain_support: &SwapchainSupport) -> Option<vk::PresentModeKHR> {
        let available_presentation_modes = &swapchain_support.present_modes[..];

        let mut has_mailbox = false;
        let mut has_fifo = false;
        for mode in available_presentation_modes {
            if *mode == vk::PresentModeKHR::MAILBOX {
                has_mailbox = true;
            } else if *mode == vk::PresentModeKHR::FIFO {
                has_fifo = true;
            }
        }

        if has_mailbox {
            Some(vk::PresentModeKHR::MAILBOX)
        } else if has_fifo {
            Some(vk::PresentModeKHR::FIFO)
        } else {
            None
        }
    }

    #[allow(unused)]
    fn choose_swapchain_extent(&self, swapchain_support: &SwapchainSupport, window: &Window) -> vk::Extent2D {
        let capabilities = swapchain_support.capabilities;
        if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            let size = window.inner_size();
            let width = u32::clamp(size.width, capabilities.min_image_extent.width, capabilities.max_image_extent.width);
            let height = u32::clamp(size.height, capabilities.min_image_extent.height, capabilities.max_image_extent.height);
            vk::Extent2D::builder()
                .width(width)
                .height(height)
                .build()
        }
    }

    fn create_swapchain(&self, inst: &Instance, window: &Window, device: &Device, app_data: &mut AppData) -> Result<()> {
        let physical_device = app_data.physical_device.unwrap();
        let swapchain_support: SwapchainSupport;
        unsafe {
            swapchain_support = SwapchainSupport::get(inst, app_data, physical_device)?;
        }

        let format = self.choose_surface_format(&swapchain_support).unwrap();
        let mode = self.choose_presentation_mode(&swapchain_support).unwrap();
        let extent = self.choose_swapchain_extent(&swapchain_support, window);

        app_data.swapchain_format = Some(format.format);
        app_data.swapchain_extent = Some(extent);

        let mut image_count = swapchain_support.capabilities.min_image_count + 1;
        if swapchain_support.capabilities.max_image_count != 0 && image_count > swapchain_support.capabilities.max_image_count {
            image_count = swapchain_support.capabilities.max_image_count;
        }

        let graphics_queue_index = app_data.graphics_queue_family.unwrap();
        let present_queue_index = app_data.present_queue_family.unwrap();
        let mut queue_family_indices = vec![];
        let mut image_sharing_mode = vk::SharingMode::EXCLUSIVE;
        if graphics_queue_index != present_queue_index {
            queue_family_indices.push(graphics_queue_index);
            queue_family_indices.push(present_queue_index);
            image_sharing_mode = vk::SharingMode::CONCURRENT;
        }

        let surface = app_data.surface.unwrap();

        let swapchain_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface)
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(image_sharing_mode)
            .queue_family_indices(&queue_family_indices)
            .pre_transform(swapchain_support.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(mode)
            .clipped(true)
            .old_swapchain(vk::SwapchainKHR::null());

        let swapchain: vk::SwapchainKHR;
        unsafe {
            debug!("Creating swapchain...");
            swapchain = device.create_swapchain_khr(&swapchain_info, None)?;
            trace!("Swapchain created: {:?}", swapchain);
        }
        app_data.swapchain = Some(swapchain);

        unsafe {
            app_data.swapchain_images = device.get_swapchain_images_khr(swapchain)?;
        }

        Ok(())
    }

    fn destroy_swapchain(&self, device: &Device, app_data: &mut AppData) -> () {
        if let Some(swapchain) = app_data.swapchain.take() {
            debug!("Destroying swapchain...");
            unsafe {
                device.destroy_swapchain_khr(swapchain, None);
            }
        }
    }

    fn create_swapchain_image_views(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        debug!("Creating swapchain image views for {} images...", app_data.swapchain_images.len());
        let format = app_data.swapchain_format.unwrap();
        let image_views = app_data.swapchain_images.iter()
            .map(|i| {
                let components = vk::ComponentMapping::builder()
                    .r(vk::ComponentSwizzle::IDENTITY)
                    .g(vk::ComponentSwizzle::IDENTITY)
                    .b(vk::ComponentSwizzle::IDENTITY)
                    .a(vk::ComponentSwizzle::IDENTITY);

                let subresource_range = vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1);

                let image_view_info = vk::ImageViewCreateInfo::builder()
                    .image(*i)
                    .view_type(vk::ImageViewType::_2D)
                    .format(format)
                    .components(components)
                    .subresource_range(subresource_range);

                unsafe {
                    device.create_image_view(&image_view_info, None)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        app_data.swapchain_image_views = image_views;
        debug!("Swapchain image views created: {:?}", app_data.swapchain_image_views);

        Ok(())
    }

    fn destroy_swapchain_image_views(&self, device: &Device, app_data: &mut AppData) -> () {
        debug!("Destroying swapchain image views...");
        unsafe {
            for image_view in app_data.swapchain_image_views.iter() {
                device.destroy_image_view(*image_view, None);
            }
        }
        app_data.swapchain_image_views.clear();
    }
}

impl BootstrapLoader for BootstrapSwapchainLoader {
    fn add_required_device_extensions(&self, required_extensions: &mut Vec<*const i8>) -> Result<()> {
        required_extensions.push(vk::KHR_SWAPCHAIN_EXTENSION.name.as_ptr());

        Ok(())
    }

    fn check_physical_device_compatibility(&self, inst: &Instance, app_data: &AppData, physical_device: vk::PhysicalDevice, properties: PhysicalDeviceProperties, _features: PhysicalDeviceFeatures) -> Result<()> {
        //Note: this method assumes the KHR_SWAPCHAIN_EXTENSION has already been checked and is present.

        let swapchain_support: SwapchainSupport;
        unsafe {
            swapchain_support = SwapchainSupport::get(inst, app_data, physical_device)?;
        }

        let format = self.choose_surface_format(&swapchain_support);
        if let None = format {
            return Err(anyhow!(GraphicsCardSuitabilityError("Physical device does not support sufficient swapchain formats.")))
        }

        let present_mode = self.choose_presentation_mode(&swapchain_support);
        if let None = present_mode {
            return Err(anyhow!(GraphicsCardSuitabilityError("Physical device does not support sufficient swapchain present modes.")))
        }

        debug!("Swapchain support determined for physical device ({} - {}). Chosen format: {:?}; chosen presentation mode: {:?}", physical_device.as_raw(), properties.device_name, format.unwrap(), present_mode.unwrap());

        Ok(())
    }

    fn after_create_logical_device(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData) -> Result<()> {
        self.create_swapchain(inst, window, device, app_data)?;
        self.create_swapchain_image_views(device, app_data)?;

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        self.destroy_swapchain_image_views(device, app_data);
        self.destroy_swapchain(device, app_data);
    }
}
