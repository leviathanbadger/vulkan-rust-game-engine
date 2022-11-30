use super::{BootstrapLoader};

use anyhow::{anyhow, Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*,
    vk::{PhysicalDeviceProperties, PhysicalDeviceFeatures, KhrSurfaceExtension, KhrSwapchainExtension},
};

use crate::{
    app::{GraphicsCardSuitabilityError},
    app_data::{AppData},
    buffer::{Image2D},
    bootstrap_loader
};

#[derive(Debug, Default)]
pub struct SwapchainInfo {
    pub surface_format: vk::SurfaceFormatKHR,
    pub present_mode: vk::PresentModeKHR,
    pub extent: vk::Extent2D,
    pub image_count: u32,
    pub swapchain: vk::SwapchainKHR,
    pub images: Vec<Image2D>
}

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

bootstrap_loader! {
    pub struct BootstrapSwapchainLoader {
        depends_on();
    }
}

impl BootstrapSwapchainLoader {
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

        let mut image_count = swapchain_support.capabilities.min_image_count + 1;
        if swapchain_support.capabilities.max_image_count != 0 && image_count > swapchain_support.capabilities.max_image_count {
            image_count = swapchain_support.capabilities.max_image_count;
        }

        let queue_info = &app_data.queue_info.as_ref().unwrap();
        let graphics_queue_index = queue_info.graphics_queue_family;
        let present_queue_index = queue_info.present_queue_family;

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

        let images: Vec<Image2D>;
        unsafe {
            let swapchain_images = device.get_swapchain_images_khr(swapchain)?;
            images = Image2D::create_from_swapchain_images(&swapchain_images[..], format.format, extent, device)?;
        }

        let mut swapchain_info = SwapchainInfo::default();
        swapchain_info.surface_format = format;
        swapchain_info.present_mode = mode;
        swapchain_info.extent = extent;
        swapchain_info.image_count = image_count;
        swapchain_info.swapchain = swapchain;
        swapchain_info.images = images;
        app_data.swapchain = Some(swapchain_info);

        Ok(())
    }

    fn destroy_swapchain(&self, device: &Device, app_data: &mut AppData) -> () {
        if let Some(mut swapchain) = app_data.swapchain.take() {
            debug!("Destroying swapchain...");

            for image in swapchain.images.iter_mut() {
                image.destroy(device);
            }

            unsafe {
                device.destroy_swapchain_khr(swapchain.swapchain, None);
            }
        }
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

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        self.destroy_swapchain(device, app_data);
    }
}
