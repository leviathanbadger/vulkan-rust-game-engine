use std::{collections::{HashSet}, os::raw::c_void, ffi::CStr};
use anyhow::{anyhow, Result};
use winit::{
    dpi::{LogicalSize},
    window::{Window, WindowBuilder},
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent}
};
use vulkanalia::{
    loader::{LibloadingLoader, LIBRARY},
    window as vk_window,
    prelude::v1_0::*, vk::ExtDebugUtilsExtension
};

const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

#[derive(Debug, Default)]
pub struct AppData {
    messenger: Option<vk::DebugUtilsMessengerEXT>
}

#[derive(Debug)]
pub struct App {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,
    pub entry: Entry,
    pub inst: Instance,
    pub app_data: AppData
}

impl App {
    pub unsafe fn create(initial_title: &str, default_size: LogicalSize<i32>) -> Result<Self> {
        debug!("Creating window and window event loop.");
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(initial_title)
            .with_inner_size(default_size)
            .build(&event_loop)?;

        let mut app_data = AppData::default();

        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let inst = Self::create_instance(initial_title, &window, &entry, &mut app_data)?;

        Ok(Self {
            event_loop: Some(event_loop),
            window,
            entry,
            inst,
            app_data
        })
    }

    extern "system" fn debug_callback(
        severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        _type: vk::DebugUtilsMessageTypeFlagsEXT,
        data: *const vk::DebugUtilsMessengerCallbackDataEXT,
        _: *mut c_void
    ) -> vk::Bool32 {
        let data = unsafe { *data };
        let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

        if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
            error!("({:?}) {}", _type, message);
        } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
            warn!("({:?}) {}", _type, message);
        } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
            debug!("({:?}) {}", _type, message);
        } else {
            trace!("({:?}) {}", _type, message);
        }

        vk::FALSE
    }

    unsafe fn create_instance(initial_title: &str, window: &Window, entry: &Entry, app_data: &mut AppData) -> Result<Instance> {
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

        let mut request_layers = vec![];

        let mut request_extensions = vk_window::get_required_instance_extensions(window)
            .iter()
            .map(|n| *n.clone())
            .collect::<Vec<_>>();

        if VALIDATION_ENABLED {
            request_layers.push(VALIDATION_LAYER);
            request_extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name);
        }

        let available_layers = entry
            .enumerate_instance_layer_properties()?
            .iter()
            .map(|l| l.layer_name)
            .collect::<HashSet<_>>();
        debug!("Available Vulkan layers: {:?}", available_layers);
        debug!("Requesting Vulkan layers: {:?}", request_layers);

        let mut request_layers_ptrs = vec![];
        for layer in request_layers {
            if !available_layers.contains(&layer) {
                return Err(anyhow!("Vulkan layer (\"{}\") requested but not supported.", layer));
            }
            request_layers_ptrs.push(layer.as_ptr());
        }

        let available_extensions = entry
            .enumerate_instance_extension_properties(None)?
            .iter()
            .map(|e| e.extension_name)
            .collect::<HashSet<_>>();
        debug!("Available Vulkan extensions: {:?}", available_extensions);
        debug!("Requesting Vulkan extensions: {:?}", request_extensions);

        let mut request_extensions_ptrs = vec![];
        for ext in request_extensions {
            if !available_extensions.contains(&ext) {
                return Err(anyhow!("Vulkan extension (\"{}\") requested but not supported.", ext));
            }
            request_extensions_ptrs.push(ext.as_ptr());
        }

        debug!("Creating Vulkan instance with requested layers and extensions.");
        let mut info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&request_layers_ptrs)
            .enabled_extension_names(&request_extensions_ptrs);

        let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
            .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
            .user_callback(Some(Self::debug_callback));

        if VALIDATION_ENABLED {
            info = info.push_next(&mut debug_info);
        }

        let inst = entry.create_instance(&info, None)?;

        if VALIDATION_ENABLED {
            debug!("Creating Vulkan debug utils messenger. Future validation/error/diagnostics from Vulkan will be logged.");
            app_data.messenger = Some(inst.create_debug_utils_messenger_ext(&debug_info, None)?);
        }

        Ok(inst)
    }

    pub fn run(mut self) -> ! {
        let mut destroying = false;
        debug!("Starting window event loop.");
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
            if let Some(messenger) = self.app_data.messenger.take() {
                debug!("Destroying Vulkan debug utils messenger. Additional Vulkan messages may not be logged");
                self.inst.destroy_debug_utils_messenger_ext(messenger, None);
            }
            debug!("Destroying Vulkan instance.");
            self.inst.destroy_instance(None);
        }
    }
}
