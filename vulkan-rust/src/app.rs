use std::{collections::{HashSet}, ffi::CStr};
use anyhow::{anyhow, Result};
use thiserror::Error;
use winit::{
    dpi::{LogicalSize},
    window::{Window, WindowBuilder},
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent}
};
use vulkanalia::{
    loader::{LibloadingLoader, LIBRARY},
    window as vk_window,
    prelude::v1_0::*,
    vk::{KhrSurfaceExtension, StringArray}
};

use crate::bootstrap::{BootstrapLoader, queue_family_indices::QueueFamilyIndices};

#[derive(Debug, Default)]
pub struct AppData {
    pub messenger: Option<vk::DebugUtilsMessengerEXT>,
    physical_device: Option<vk::PhysicalDevice>,
    graphics_queue: Option<vk::Queue>,
    present_queue: Option<vk::Queue>,
    pub surface: Option<vk::SurfaceKHR>
}

#[derive(Debug, Error)]
#[error("Missing {0}")]
pub struct GraphicsCardSuitabilityError(pub &'static str);

#[derive(Debug)]
pub struct App {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,
    pub app_data: AppData,
    //entry can't be disposed of before inst or it causes a segmentation fault when you attempt to use inst
    #[allow(unused)]
    entry: Entry,
    pub inst: Instance,
    pub device: Device,
    pub bootstrap_loaders: Vec<Box<dyn BootstrapLoader>>
}

impl App {
    pub fn create(initial_title: &'static str, default_size: LogicalSize<i32>, bootstrap_loaders: Vec<Box<dyn BootstrapLoader>>) -> Result<Self> {
        debug!("Creating window and window event loop.");
        let event_loop = EventLoop::new();
        //TODO: add support for fullscreen
        let window = WindowBuilder::new()
            .with_title(initial_title)
            .with_inner_size(default_size)
            .build(&event_loop)?;

        let mut app_data = AppData::default();

        let inst: Instance;
        let entry: Entry;
        unsafe {
            let loader = LibloadingLoader::new(LIBRARY)?;
            entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
            inst = Self::create_instance(initial_title, &bootstrap_loaders, &window, &mut app_data, &entry)?;
        }

        unsafe {
            debug!("Creating Vulkan surface KHR.");
            app_data.surface = Some(vk_window::create_surface(&inst, &window)?);
        }

        let device: Device;
        unsafe {
            debug!("Selecting graphics card (physical device) and creating logical device.");

            let mut request_layers_ptrs = vec![];
            let mut request_extensions_ptrs = vec![];
            for loader in bootstrap_loaders.iter() {
                loader.add_required_device_layers(&mut request_layers_ptrs)?;
                loader.add_required_device_extensions(&mut request_extensions_ptrs)?;
            }

            Self::select_graphics_card(&inst, &bootstrap_loaders, &mut app_data, &request_layers_ptrs, &request_extensions_ptrs)?;
            device = Self::create_logical_device(&inst, &bootstrap_loaders, &mut app_data, &request_layers_ptrs, &request_extensions_ptrs)?;
        }

        Ok(Self {
            event_loop: Some(event_loop),
            window,
            app_data,
            entry,
            inst,
            device,
            bootstrap_loaders: bootstrap_loaders
        })
    }

    unsafe fn create_instance<'a>(initial_title: &str, bootstrap_loaders: &Vec<Box<dyn BootstrapLoader>>, window: &Window, app_data: &mut AppData, entry: &Entry) -> Result<Instance> {
        debug!("Selecting instance extensions and layers, and creating instance.");
        let mut zero_terminated: String = "".to_owned();
        zero_terminated.push_str(initial_title);
        zero_terminated.push_str("\0");

        let app_info = vk::ApplicationInfo::builder()
            .application_name(zero_terminated.as_bytes())
            .application_version(vk::make_version(0, 0, 1))
            .engine_name(b"No engine\0")
            .engine_version(vk::make_version(0, 0, 1))
            .api_version(vk::make_version(0, 0, 1));
        trace!("Vulkan app_info created to present to create_instance. App Name: {:?}; Engine Name: {:?}", CStr::from_ptr(app_info.application_name), CStr::from_ptr(app_info.engine_name));

        let mut request_layers_ptrs = vec![];

        let mut request_extensions_ptrs = vk_window::get_required_instance_extensions(window)
            .iter()
            .map(|n| n.as_ptr())
            .collect::<Vec<_>>();

        for loader in bootstrap_loaders.iter() {
            loader.add_required_instance_layers(&mut request_layers_ptrs)?;
            loader.add_required_instance_extensions(&mut request_extensions_ptrs)?;
        }

        Self::check_instance(entry, &request_layers_ptrs, &request_extensions_ptrs)?;

        debug!("Creating Vulkan instance with requested layers and extensions.");
        let inst_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&request_layers_ptrs)
            .enabled_extension_names(&request_extensions_ptrs);

        let last_callback = move |inst_info: vk::InstanceCreateInfoBuilder| -> Result<Instance> {
            trace!("Final callback. Creating Vulkan instance");
            let inst = entry.create_instance(&inst_info, None)?;
            debug!("Vulkan instance created: {:?}", inst);
            Ok(inst)
        };

        fn create_and_invoke_callback(index: usize, bootstrap_loaders: &Vec<Box<dyn BootstrapLoader>>, app_data: &mut AppData, last_callback: &dyn Fn(vk::InstanceCreateInfoBuilder) -> Result<Instance>, inst_info: vk::InstanceCreateInfoBuilder) -> Result<Instance> {
            trace!("Invoking callback for index {} to create Vulkan instance", index);
            let loader_res = bootstrap_loaders.get(index);
            match loader_res {
                Some(loader) => {
                    let next_callback = |inst_info: vk::InstanceCreateInfoBuilder, app_data: &mut AppData| create_and_invoke_callback(index + 1, bootstrap_loaders, app_data, last_callback, inst_info);
                    loader.instance_create(inst_info, app_data, &next_callback)
                },
                None => {
                    last_callback(inst_info)
                }
            }
        }

        let inst = create_and_invoke_callback(0, bootstrap_loaders, app_data, &last_callback, inst_info)?;

        Ok(inst)
    }

    unsafe fn check_instance(entry: &Entry, request_layers_ptrs: &Vec<*const i8>, request_extensions_ptrs: &Vec<*const i8>) -> Result<()> {
        let available_layers = entry
            .enumerate_instance_layer_properties()?
            .iter()
            .map(|l| l.layer_name)
            .collect::<HashSet<_>>();
        let request_layers = request_layers_ptrs
            .iter()
            .map(|name| CStr::from_ptr(*name))
            .collect::<Vec<_>>();
        debug!("Available Vulkan layers: {:?}", available_layers);
        info!("Requesting Vulkan layers: {:?}", request_layers);

        for layer in request_layers {
            if !available_layers.contains(&StringArray::from_cstr(layer)) {
                return Err(anyhow!("Vulkan layer ({:?}) requested but not supported.", layer));
            }
        }

        let available_extensions = entry
            .enumerate_instance_extension_properties(None)?
            .iter()
            .map(|e| e.extension_name)
            .collect::<HashSet<_>>();
        let request_extensions = request_extensions_ptrs
            .iter()
            .map(|name| CStr::from_ptr(*name))
            .collect::<Vec<_>>();
        debug!("Available Vulkan extensions: {:?}", available_extensions);
        info!("Requesting Vulkan extensions: {:?}", request_extensions);

        for ext in request_extensions {
            if !available_extensions.contains(&StringArray::from_cstr(ext)) {
                return Err(anyhow!("Vulkan extension ({:?}) requested but not supported.", ext));
            }
        }

        Ok(())
    }

    unsafe fn select_graphics_card(inst: &Instance, bootstrap_loaders: &Vec<Box<dyn BootstrapLoader>>, app_data: &mut AppData, request_layers_ptrs: &Vec<*const i8>, request_extensions_ptrs: &Vec<*const i8>) -> Result<()> {
        let physical_devices = inst.enumerate_physical_devices()?;

        for physical_device in physical_devices {
            let properties = inst.get_physical_device_properties(physical_device);

            if let Err(error) = Self::check_graphics_card(inst, bootstrap_loaders, app_data, physical_device, request_layers_ptrs, request_extensions_ptrs) {
                warn!("Skipping graphics card ({} - {}): {}", physical_device.as_raw(), properties.device_name, error);
            } else {
                //TODO: select _best_ graphics card, not just the first one in the list
                info!("Using graphics card ({} - {}).", physical_device.as_raw(), properties.device_name);
                app_data.physical_device = Some(physical_device);
                return Ok(());
            }
        }

        Err(anyhow!(GraphicsCardSuitabilityError("No suitable graphics card was found")))
    }

    unsafe fn check_graphics_card(inst: &Instance, bootstrap_loaders: &Vec<Box<dyn BootstrapLoader>>, app_data: &AppData, physical_device: vk::PhysicalDevice, request_layers_ptrs: &Vec<*const i8>, request_extensions_ptrs: &Vec<*const i8>) -> Result<()> {
        let properties = inst.get_physical_device_properties(physical_device);
        let features = inst.get_physical_device_features(physical_device);

        //Check for layers and extensions before calling check_physical_device_compatibility. Some bootstrap loaders assume their requests extension are already confirmed to be present
        Self::check_physical_device(inst, physical_device, request_layers_ptrs, request_extensions_ptrs, false)?;

        for loader in bootstrap_loaders.iter() {
            loader.check_physical_device_compatibility(inst, app_data, physical_device, properties, features)?;
        }

        let queue_family_indices = QueueFamilyIndices::get(inst, app_data, physical_device)?;
        if let None = queue_family_indices.graphics {
            return Err(anyhow!(GraphicsCardSuitabilityError("No queue family on this physical device supports graphics operations.")));
        }
        if let None = queue_family_indices.present {
            return Err(anyhow!(GraphicsCardSuitabilityError("No queue family on this physical device supports KHR present operations.")));
        }

        Ok(())
    }

    unsafe fn create_logical_device(inst: &Instance, bootstrap_loaders: &Vec<Box<dyn BootstrapLoader>>, app_data: &mut AppData, request_layers_ptrs: &Vec<*const i8>, request_extensions_ptrs: &Vec<*const i8>) -> Result<Device> {
        let physical_device = app_data.physical_device.unwrap();
        let indices = QueueFamilyIndices::get(&inst, app_data, physical_device)?;

        let mut unique_queue_family_indices = HashSet::new();
        let graphics_queue_index = indices.graphics.unwrap();
        let present_queue_index = indices.present.unwrap();
        unique_queue_family_indices.insert(graphics_queue_index);
        unique_queue_family_indices.insert(present_queue_index);

        let queue_priorities = &[1.0];
        let queue_infos = unique_queue_family_indices
            .iter()
            .map(|i| {
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(*i)
                    .queue_priorities(queue_priorities)
            })
            .collect::<Vec<_>>();

        let mut features = vk::PhysicalDeviceFeatures::builder();

        for loader in bootstrap_loaders.iter() {
            loader.add_required_device_features(&mut features)?;
        }

        // Sanity check. At this point the physical device should have been selected based (in part) on having the requested layers and extensions
        Self::check_physical_device(inst, physical_device, request_layers_ptrs, request_extensions_ptrs, true)?;

        debug!("Creating Vulkan logical device with requested layers and features.");
        let device_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_layer_names(&request_layers_ptrs)
            .enabled_extension_names(&request_extensions_ptrs)
            .enabled_features(&features);

        let device = inst.create_device(physical_device, &device_info, None)?;

        let graphics_queue = device.get_device_queue(graphics_queue_index, 0);
        app_data.graphics_queue = Some(graphics_queue);
        debug!("Vulkan graphics queue handle: {}", graphics_queue.as_raw());

        let present_queue = device.get_device_queue(present_queue_index, 0);
        app_data.present_queue = Some(present_queue);
        debug!("Vulkan KHR present queue handle: {}", present_queue.as_raw());

        Ok(device)
    }

    unsafe fn check_physical_device(inst: &Instance, physical_device: vk::PhysicalDevice, request_layers_ptrs: &Vec<*const i8>, request_extensions_ptrs: &Vec<*const i8>, log_check: bool) -> Result<()> {
        let available_layers = inst
            .enumerate_device_layer_properties(physical_device)?
            .iter()
            .map(|l| l.layer_name)
            .collect::<HashSet<_>>();
        let request_layers = request_layers_ptrs
            .iter()
            .map(|name| CStr::from_ptr(*name))
            .collect::<Vec<_>>();
        if log_check {
            debug!("Available Vulkan device layers: {:?}", available_layers);
            info!("Requesting Vulkan device layers: {:?}", request_layers);
        }

        for layer in request_layers {
            if !available_layers.contains(&StringArray::from_cstr(layer)) {
                return Err(anyhow!("Vulkan device layer ({:?}) requested but not supported.", layer));
            }
        }

        let available_extensions = inst
            .enumerate_device_extension_properties(physical_device, None)?
            .iter()
            .map(|l| l.extension_name)
            .collect::<HashSet<_>>();
        let request_extensions = request_extensions_ptrs
            .iter()
            .map(|name| CStr::from_ptr(*name))
            .collect::<Vec<_>>();
        if log_check {
            debug!("Available Vulkan device extensions: {:?}", available_extensions);
            info!("Requesting Vulkan device extensions: {:?}", request_extensions);
        }

        for ext in request_extensions {
            if !available_extensions.contains(&StringArray::from_cstr(ext)) {
                return Err(anyhow!("Vulkan device extension ({:?}) requested but not supported.", ext));
            }
        }

        Ok(())
    }

    //Deliberately not a ref, because the run method needs to own "self"
    pub fn run(mut self) -> ! {
        let mut destroying = false;
        info!("Starting window event loop.");
        //TODO: Don't abuse Option<> in the struct in order to call run on the event loop without causing an ownership error
        //TODO: Add Ctrl+C handler to gracefully shut down app
        let event_loop = self.event_loop.take().unwrap();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::MainEventsCleared if !destroying => {
                    self.render().unwrap();
                }
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    info!("Window close requested. Shutting down application...");
                    destroying = true;
                    *control_flow = ControlFlow::Exit;
                    self.destroy();
                }
                _ => { }
            }
        });
    }

    pub fn render(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn destroy(&mut self) {
        unsafe {
            debug!("Destroying Vulkan logical device.");
            self.device.device_wait_idle().unwrap();
            self.device.destroy_device(None);

            if let Some(surface) = self.app_data.surface.take() {
                debug!("Destroying Vulkan surface KHR");
                self.inst.destroy_surface_khr(surface, None);
            }

            for loader in self.bootstrap_loaders.iter().rev() {
                loader.before_destroy_instance(&self.inst, &mut self.app_data);
            }

            debug!("Destroying Vulkan instance.");
            self.inst.destroy_instance(None);
        }
    }
}
