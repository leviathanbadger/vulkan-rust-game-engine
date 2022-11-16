use crate::app::{AppData, GraphicsCardSuitabilityError};

use super::BootstrapLoader;

use anyhow::{anyhow, Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*,
    vk::{PhysicalDeviceProperties, PhysicalDeviceFeatures, KhrSurfaceExtension},
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
}
